use std::cmp::PartialEq;
use crate::ui::framebuffer::FrameBuffer;
use crate::ui::region::Region;
use crossterm::event::{Event, poll, read, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen};
use std::fmt::{Display, Formatter};
use std::io;
use std::time::Duration;
use crossterm::event::KeyCode::Char;
use crossterm::execute;
use crate::utils::logger::info;

#[derive(Clone, PartialEq)]
pub enum Colour {
    Green,
    Blue,
    Purple,
    White,
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
                Colour::White => "\x1b[37m",
            }
        )
    }
}

pub struct TUI {
    width: u16,
    height: u16,
    regions: Vec<Region>,
    current: FrameBuffer,
    previous: FrameBuffer,
    run: bool
}

impl TUI {
    pub fn new() -> Result<TUI, String> {
        let (rows, cols) = match size() {
            Ok(res) => res,
            Err(err) => {
                return Err(format!(
                    "Could not get terminal size to init TUI. ({})",
                    err
                ));
            }
        };

        Self::enter_tui_mode();

        let regions = Self::init_regions(cols, rows)?;

        Ok(TUI {
            width: cols,
            height: rows,
            regions,
            current: FrameBuffer::new(cols, rows),
            previous: FrameBuffer::new(cols, rows),
            run: true
        })
    }

    fn init_regions(width: u16, height: u16) -> Result<Vec<Region>, String> {
        // input region
        let input_region = Region {
            x: 0,
            y: 0,
            height: (height as f64 * 0.1).ceil() as u16,
            width,
            data: Vec::new(),
            border_colour: Colour::Green,
        };
        // data region
        let data_region = Region {
            x: 0,
            y: input_region.height,
            height: (height as f64 * 0.7).ceil() as u16,
            width,
            data: Vec::new(),
            border_colour: Colour::Blue,
        };
        // log region
        let log_region = Region {
            x: 0,
            y: data_region.y + data_region.height,
            height: height - (data_region.height + input_region.height),
            width,
            data: Vec::new(),
            border_colour: Colour::Purple,
        };

        Ok(vec![input_region, data_region, log_region])
    }

    pub fn start(&mut self) -> Result<(), String> {
        // enter alternate display buffer
        Self::enter_tui_mode()?;

        loop {
            if !self.run {
                Self::leave_tui_mode()?;
                info!("Exiting");
                return Ok(())
            }

            self.draw();

            if poll(Duration::from_millis(100)).map_err(|x| x.to_string())? {
                self.handle_event(read().map_err(|x| x.to_string())?)
            }
        }
    }

    fn draw(&mut self) {
        for region in &self.regions {
            region.draw(&mut self.current)
        }

        self.flush_diff()
    }

    fn flush_diff(&mut self) {
        // \x1b[3;6H\x1b[37m@

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.current.get(x, y);
                if self.previous.get(x, y) != cell {
                    print!(
                        "\x1b[{};{}H{}{}",
                        y + 1,
                        x + 1,
                        cell.colour.to_string(),
                        cell.char
                    );
                }
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(res) => {
                println!("pressed key {}", res.code);
                if res.code == Char('c') && res.modifiers.contains(KeyModifiers::CONTROL)  {
                    self.run = false;
                }
            }
            Event::Mouse(res) => {}
            Event::Resize(cols, rows) => {
                self.height = rows;
                self.width = cols;
            }
            _ => {}
        }
    }

    pub fn enter_tui_mode() -> Result<(), String> {
        execute!(io::stdout(), EnterAlternateScreen).map_err(|x| x.to_string())?;
        enable_raw_mode().map_err(|x| x.to_string())?;
        Ok(())
    }
    pub fn leave_tui_mode() -> Result<(), String> {

        // Do anything on the alternate screen

        execute!(io::stdout(), LeaveAlternateScreen).map_err(|x| x.to_string())?;
        disable_raw_mode().map_err(|x| x.to_string())?;
        Ok(())
    }
}
