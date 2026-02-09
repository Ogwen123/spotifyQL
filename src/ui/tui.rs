use crossterm::event::{Event, poll, read};
use crossterm::terminal::size;
use std::io::Write;
use std::time::Duration;

static GREEN: &str = "\x1b[32m";
static BLUE: &str = "\x1b[34m";
static PURPLE: &str = "\x1b[35m";

pub struct Region {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    data: Vec<String>, // vector of lines to be displayed
}

impl Region {
    fn build_inner_buffer(&self) -> Result<Vec<String>, String> {
        Ok(Vec::new())
    }

    /// Make a border of the given colour and fill with inner buffer (buffer length is 0)
    fn build_region_buffer(&self, colour: &str) -> Result<(), String> {
        let mut buffer: Vec<String> = Vec::new();

        for i in 0..self.height {
            if i == 0 {
                buffer.push(format!("╭{:─<w$}╮", "", w = self.width as usize))
            } else if i == self.height - 1 {
                buffer.push(format!("╰{:─<w$}╯", "", w = self.width as usize))
            } else {
                buffer.push(format!("│{: <w$}│", "", w = self.width as usize))
            }
        }

        Ok(())
    }
}

pub struct TUI {
    width: u16,
    height: u16,
    regions: Vec<Region>,
    dirty: bool,
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

        let regions = Self::init_regions(cols, rows)?;

        Ok(TUI {
            width: cols,
            height: rows,
            regions,
            dirty: true,
        })
    }

    pub fn init_regions(width: u16, height: u16) -> Result<Vec<Region>, String> {
        // input region
        let input_region = Region {
            x: 0,
            y: 0,
            height: (height as f64 * 0.1).ceil() as u16,
            width,
            data: Vec::new(),

        };
        // data region
        let data_region = Region {
            x: 0,
            y: input_region.height,
            height: (height as f64 * 0.7).ceil() as u16,
            width,
            data: Vec::new(),

        };
        // log region
        let log_region = Region {
            x: 0,
            y: data_region.y + data_region.height,
            height: height - (data_region.height + input_region.height),
            width,
            data: Vec::new(),
        };

        Ok(vec![input_region, data_region, log_region])
    }

    pub fn start(&mut self) -> Result<(), String> {
        // enter alternate display buffer
        println!("\x1B[?1049h");

        loop {
            if self.dirty {
                self.display();
                self.dirty = false;
            }

            if poll(Duration::from_millis(100)).map_err(|x| x.to_string())? {
                self.handle_event(read().map_err(|x| x.to_string())?)
            }
        }
    }

    /// Combine regions into a single string
    fn build_buffer(&self) -> String {
        let buffer: Vec<String> = Vec::new();

        buffer.join("\n")
    }

    fn display(&self) {
        let buffer = self.build_buffer();

        println!("{}", buffer)
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(res) => {
                println!("pressed key {}", res.code)
            }
            Event::Mouse(res) => {}
            Event::Resize(cols, rows) => {
                self.height = rows;
                self.width = cols;
            }
            _ => {}
        }
    }

    pub fn leave_alternate_buffer() {
        println!("\x1B[?1049l")
    }
}
