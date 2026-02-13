use std::rc::Rc;
use crate::ui::framebuffer::{Cell, FrameBuffer};
use crate::ui::regions::region::{Region, RegionData, RegionType};
use crate::ui::tui::{Colour, Log, Severity};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crate::app_context::AppContext;
use crate::query::run::run_query;
use crate::utils::utils::bounds_loc;

pub struct InputRegion {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub border_colour: Colour,
    pub focused_border_colour: Colour,
    pub value: String, // vector of lines to be displayed
    pub focused: bool,
    pub placeholder: String,
}

impl Region for InputRegion {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        let mut buffer = vec![
            Cell {
                char: ' ',
                colour: Colour::White,
                bold: false
            };
            ((self.width - 2) * (self.height - 2)) as usize
        ];

        if self.value.len() == 0 {
            for (index, char) in self.placeholder.chars().enumerate() {
                buffer[index] = Cell {
                    char,
                    colour: Colour::Grey,
                    bold: false
                }
            }
        } else {
            for (index, char) in self.value.chars().enumerate() {
                buffer[index] = Cell {
                    char,
                    colour: Colour::White,
                    bold: false
                }
            }
        }

        buffer
    }

    /// Make a border of the given colour and fill with inner buffer (buffer length is 0)
    fn build_region_buffer(&self) -> Vec<Cell> {
        let mut buffer: Vec<Cell> = Vec::new();
        let inner_buffer = self.build_inner_buffer();

        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.focused {self.focused_border_colour.clone()} else {self.border_colour.clone()};

                if y == 0 {
                    if x == 0 {
                        buffer.push(Cell {
                            char: '╭',
                            colour: c,
                            bold: self.focused
                        })
                    } else if x == self.width - 1 {
                        buffer.push(Cell {
                            char: '╮',
                            colour: c,
                            bold: self.focused
                        })
                    } else {
                        buffer.push(Cell {
                            char: '─',
                            colour: c,
                            bold: self.focused
                        })
                    }
                } else if y == self.height - 1 {
                    if x == 0 {
                        buffer.push(Cell {
                            char: '╰',
                            colour: c,
                            bold: self.focused
                        })
                    } else if x == self.width - 1 {
                        buffer.push(Cell {
                            char: '╯',
                            colour: c,
                            bold: self.focused
                        })
                    } else {
                        buffer.push(Cell {
                            char: '─',
                            colour: c,
                            bold: self.focused
                        })
                    }
                } else {
                    if x == 0 || x == self.width - 1 {
                        buffer.push(Cell {
                            char: '│',
                            colour: c,
                            bold: self.focused
                        })
                    } else {
                        buffer.push(
                            inner_buffer[((y - 1) * (self.width - 2) + (x - 1)) as usize].clone(),
                        )
                    }
                }
            }
        }

        buffer
    }

    fn draw(&self, fb: &mut FrameBuffer) {
        let content = self.build_region_buffer();

        for (loc, cell) in content.into_iter().enumerate() {
            let local_y = (loc as f64 / self.width as f64).floor() as u16;
            let local_x = loc as u16 % self.width;

            fb.put(local_x + self.x, local_y + self.y, cell)
        }
    }

    fn handle_event(&mut self, event: Event, cx: &mut AppContext, lb: &mut Vec<Log>) {
        if !self.focused {return}

        match event {
            Event::Key(res) => {
                match res.code {
                    KeyCode::Char(c) => {
                        self.value.push(c);
                    },
                    KeyCode::Backspace => {
                        self.value.pop();
                    },
                    KeyCode::Enter => {
                        // run query
                        match run_query(cx, self.value.clone()) {
                            Ok(_) => {},
                            Err(err) => lb.push(Log {
                                severity: Severity::Error,
                                content: err
                            })
                        };
                        self.value = String::new()
                    }
                    _ => {}
                }
            }
            Event::Mouse(res) => {

            }
            _ => {}
        }

    }

    fn _debug(&self) {
        println!(
            "width: {}, height: {}, x: {}, y: {}",
            self.width, self.height, self.x, self.y
        );
    }

    fn bounds_loc(&self, x: u16, y: u16) -> bool {
        bounds_loc(self.x, self.y, self.width, self.height, x, y)
    }

    fn set_focus(&mut self, focus: bool) {
        self.focused = focus;
    }

    fn _type(&self) -> RegionType {
        RegionType::Input
    }

    fn send_data(&mut self, data: RegionData) {}
}
