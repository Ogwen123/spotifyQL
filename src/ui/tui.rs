use crate::ui::framebuffer::FrameBuffer;
use crate::ui::regions::input_region::InputRegion;
use crate::ui::regions::list_region::ListRegion;
use crate::ui::regions::region::{Region, RegionData, RegionType};
use crate::ui::regions::table_region::TableRegion;
use crate::utils::logger::info;
use crossterm::cursor::{Hide, Show};
use crossterm::event::KeyCode::Char;
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyModifiers, MouseEventKind, poll, read,
};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;
use std::mem::{discriminant, swap};
use std::time::Duration;
use crate::app_context::AppContext;
use crate::query::data::load_data_source;
use crate::query::parse::parse;
use crate::query::run::{run_query, TUIQueryStage};
use crate::query::tokenise::tokenise;
use crate::ui::event_action::Action;

#[derive(Clone, PartialEq)]
pub enum Colour {
    Green,
    Blue,
    Purple,
    Red,
    Cyan,
    White,
    Grey,
    BrightGreen,
    BrightBlue,
    BrightPurple,
}

impl Colour {
    pub fn code(&self) -> String {
        match self {
            Colour::Green => "32m",
            Colour::Blue => "34m",
            Colour::Purple => "35m",
            Colour::Red => "31m",
            Colour::Cyan => "36m",
            Colour::White => "37m",
            Colour::Grey => "90m",
            Colour::BrightGreen => "92m",
            Colour::BrightBlue => "94m",
            Colour::BrightPurple => "95m",
        }
        .to_string()
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Colour::Green => "\x1b[32m",
                Colour::Blue => "\x1b[34m",
                Colour::Purple => "\x1b[35m",
                Colour::Red => "\x1b[31m",
                Colour::Cyan => "\x1b[36m",
                Colour::White => "\x1b[37m",
                Colour::Grey => "\x1b[90m",
                Colour::BrightGreen => "\x1b[92m",
                Colour::BrightBlue => "\x1b[94m",
                Colour::BrightPurple => "\x1b[95m",
            }
        )
    }
}

#[derive(Clone)]
pub enum Severity {
    Log,
    Success,
    Error
}

impl Severity {
    pub fn colour(&self) -> Colour {
        match self {
            Severity::Error => Colour::Red,
            Severity::Log => Colour::Blue,
            Severity::Success => Colour::Green
        }
    }
}

#[derive(Clone)]
pub struct Log {
    pub(crate) severity: Severity,
    pub(crate) content: String
}


pub struct TUI {
    width: u16,
    height: u16,
    regions: Vec<Box<dyn Region>>,
    current: FrameBuffer,
    previous: FrameBuffer,
    run: bool,
    external_log_buffer: Vec<Log>
}

impl TUI {
    pub fn new() -> Result<TUI, String> {
        Self::enter_tui_mode()?; // need to enter alt buffer here to get correct size

        let (cols, rows) = match size() {
            Ok(res) => res,
            Err(err) => {
                return Err(format!(
                    "Could not get terminal size to init TUI. ({})",
                    err
                ));
            }
        };

        Ok(TUI {
            width: cols,
            height: rows,
            regions: Vec::new(),
            current: FrameBuffer::new(cols, rows),
            previous: FrameBuffer::new(cols, rows),
            run: true,
            external_log_buffer: Vec::new()
        }
        .init_regions(cols, rows))
    }

    fn init_regions(mut self, width: u16, height: u16) -> Self {
        // input region
        let input_region = InputRegion {
            x: 0,
            y: 0,
            height: 3,
            width,
            value: String::from("SELECT * FROM PLAYLIST(All);"),
            border_colour: Colour::Cyan,
            focused_border_colour: Colour::Green,
            focused: true,
            placeholder: String::from("Start typing query here..."),
        };
        // data region
        let data_region = TableRegion {
            x: 0,
            y: input_region.height,
            height: (height as f64 * 0.7).ceil() as u16,
            width,
            formatted_table: Vec::new(),
            border_colour: Colour::Blue,
            focused_border_colour: Colour::BrightBlue,
            focused: false,
            vertical_scroll: 0,
            horizontal_scroll: 0
        };
        // log region
        let log_region = ListRegion {
            x: 0,
            y: data_region.y + data_region.height,
            height: height - (data_region.height + input_region.height),
            width,
            data: Vec::new(),
            border_colour: Colour::Purple,
            focused_border_colour: Colour::BrightPurple,

            focused: false,
        };

        self.regions = vec![
            Box::new(input_region),
            Box::new(data_region),
            Box::new(log_region),
        ];

        self
    }

    pub fn run(&mut self, cx: &mut AppContext) -> Result<(), String> {
        // enter alternate display buffer
        Self::enter_tui_mode()?;

        let mut log_buffer: Vec<Log> = Vec::new();
        let mut query_stage: TUIQueryStage = TUIQueryStage::NotRunning;

        loop {
            // handle loop ending
            if !self.run {
                Self::leave_tui_mode()?;
                info!("Exiting");
                return Ok(());
            }

            // check for and handle any active query processing
            match query_stage.clone() {
                TUIQueryStage::Queued(query) => {
                    log_buffer.push(Log {severity: Severity::Log, content: "Tokenising".to_string()});
                    match tokenise(query.clone()) {
                        Ok(res) => {
                            query_stage = TUIQueryStage::Tokenised(res);
                            log_buffer.push(Log {severity: Severity::Success, content: "Tokenised".to_string()});
                        },
                        Err(err) => {
                            log_buffer.push(Log {severity: Severity::Error, content: err});
                            query_stage = TUIQueryStage::NotRunning
                        }
                    }
                },
                TUIQueryStage::Tokenised(tokens) => {
                    log_buffer.push(Log {severity: Severity::Log, content: "Parsing".to_string()});
                    match parse(tokens.clone()) {
                        Ok(res) => {
                            query_stage = TUIQueryStage::Parsed(res);
                            log_buffer.push(Log {severity: Severity::Success, content: "Parsed".to_string()});

                        },
                        Err(err) => {
                            log_buffer.push(Log {severity: Severity::Error, content: err});
                            query_stage = TUIQueryStage::NotRunning
                        }
                    }
                },
                TUIQueryStage::Parsed(statement) => {
                    log_buffer.push(Log {severity: Severity::Log, content: "Loading Data".to_string()});
                    match load_data_source(cx, statement.clone().source){
                        Ok(_) => {
                            query_stage = TUIQueryStage::ParsedWithData(statement);
                            log_buffer.push(Log {severity: Severity::Success, content: "Loaded Data".to_string()});

                        },
                        Err(err) => {
                            log_buffer.push(Log {severity: Severity::Error, content: err});
                            query_stage = TUIQueryStage::NotRunning
                        }
                    }
                },
                TUIQueryStage::ParsedWithData(statement) => {
                    match statement.clone().run(cx, Some(self)) {
                        Ok(_) => query_stage = TUIQueryStage::NotRunning,
                        Err(err) => {
                            log_buffer.push(Log {severity: Severity::Error, content: err});
                            query_stage = TUIQueryStage::NotRunning
                        }
                    }
                },
                _ => {}
            }

            // handle any queued logs
            log_buffer.append(&mut self.external_log_buffer);
            self.external_log_buffer = Vec::new();
            if log_buffer.len() != 0 {
                for i in &mut self.regions {
                    if i._type() == RegionType::List {
                        i.send_data(RegionData::List(log_buffer.clone()));
                    }
                }
                log_buffer = Vec::new()
            }

            // draw regions
            self.draw();

            // handle events
            if poll(Duration::from_millis(100)).map_err(|x| x.to_string())? {
                self.handle_event(read().map_err(|x| x.to_string())?, cx, &mut log_buffer, &mut query_stage)
            }
        }
    }

    fn draw(&mut self) {
        for region in &self.regions {
            region.draw(&mut self.current)
        }

        self.flush_diff();

        swap(&mut self.current, &mut self.previous)
    }

    fn flush_diff(&mut self) {
        // \x1b[3;6H\x1b[37m@

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.current.get(x, y);
                let prev_cell = self.previous.get(x, y);
                if prev_cell.colour != cell.colour || prev_cell.char != cell.char {
                    print!(
                        "\x1b[{};{}H\x1b[{};{}{}",
                        y + 1,
                        x + 1,
                        if cell.bold { "1" } else { "22" },
                        cell.colour.code(),
                        cell.char
                    );
                }
            }
        }

        io::stdout().flush().unwrap();
    }

    fn handle_event(&mut self, event: Event, cx: &mut AppContext, lb: &mut Vec<Log>, query_stage: &mut TUIQueryStage) {
        match event {
            Event::Key(res) => {
                if res.code == Char('c') && res.modifiers.contains(KeyModifiers::CONTROL) {
                    self.run = false;
                    return;
                }
                let mut query: Option<String> = None;

                for i in self.regions.iter_mut() {
                    match i.handle_event(event.clone(), lb) {
                        Action::RunQuery(q) => {
                            query = Some(q);
                        },
                        Action::Internal => {}
                    }
                }

                if let Some(q) = query && discriminant(query_stage) == discriminant(&TUIQueryStage::NotRunning) {
                    *query_stage = TUIQueryStage::Queued(q)
                }
            }
            Event::Mouse(res) => {
                if let MouseEventKind::Down(button) = res.kind
                    && button.is_left()
                {
                    for i in self.regions.iter_mut() {
                        if i.bounds_loc(res.column, res.row) {
                            i.set_focus(true);
                        } else {
                            i.set_focus(false);
                        }
                    }
                } else {
                    for i in self.regions.iter_mut() {
                        match i.handle_event(event.clone(), lb) {
                            Action::RunQuery(_) => {},
                            Action::Internal => {}
                        }
                    }
                }
            }
            Event::Resize(cols, rows) => {
                self.height = rows;
                self.width = cols;
            }
            _ => {}
        }
    }

    pub fn enter_tui_mode() -> Result<(), String> {
        execute!(io::stdout(), EnterAlternateScreen).map_err(|x| x.to_string())?;
        execute!(io::stdout(), EnableMouseCapture).map_err(|x| x.to_string())?;
        execute!(io::stdout(), Hide).map_err(|x| x.to_string())?;
        enable_raw_mode().map_err(|x| x.to_string())?;
        Ok(())
    }
    pub fn leave_tui_mode() -> Result<(), String> {
        // Do anything on the alternate screen

        execute!(io::stdout(), LeaveAlternateScreen).map_err(|x| x.to_string())?;
        execute!(io::stdout(), DisableMouseCapture).map_err(|x| x.to_string())?;
        execute!(io::stdout(), Show).map_err(|x| x.to_string())?;
        disable_raw_mode().map_err(|x| x.to_string())?;
        Ok(())
    }

    pub fn send_table_data(&mut self, data: Vec<String>) -> Result<(), String> {
        for i in &mut self.regions {
            if i._type() == RegionType::Table {
                i.send_data(RegionData::Table(data.clone()))
            }
        }

        Ok(())
    }

    pub fn log(&mut self, log: Log) {
        self.external_log_buffer.push(log)
    }
}
