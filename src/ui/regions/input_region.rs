use crate::ui::framebuffer::{Cell, FrameBuffer};
use crate::ui::regions::region::{Region, RegionData, RegionType, REGION_NAME_PADDING};
use crate::ui::tui::{Colour, Log};
use crossterm::event::{Event, KeyCode};
use crate::ui::event_action::Action;
use crate::utils::utils::bounds_loc;

#[derive(Clone)]
pub struct InputRegion {
    pub name: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub border_colour: Colour,
    pub focused_border_colour: Colour,
    pub value: String, // vector of lines to be displayed
    pub value_stack: Vec<String>,
    /// stack_pos is 1 indexed as stack_pos == 0 means just a normal value
    pub stack_pos: usize,
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

        let name_chars = self.name.chars().collect::<Vec<char>>();

        for y in 0..self.height {
            for x in 0..self.width {
                let c = if self.focused {
                    self.focused_border_colour.clone()
                } else {
                    self.border_colour.clone()
                };

                if y == 0 {
                    if x == 0 {
                        buffer.push(Cell {
                            char: '╭',
                            colour: c,
                            bold: self.focused,
                        })
                    } else if x == self.width - 1 {
                        buffer.push(Cell {
                            char: '╮',
                            colour: c,
                            bold: self.focused,
                        })
                    } else {
                        let char = if x >= REGION_NAME_PADDING && x-REGION_NAME_PADDING < name_chars.len() as u16 {name_chars[(x-REGION_NAME_PADDING) as usize]} else {'─'};

                        buffer.push(Cell {
                            char,
                            colour: c,
                            bold: self.focused,
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

    fn handle_event(&mut self, event: Event, lb: &mut Vec<Log>) -> Action {
        if !self.focused {return Action::Internal}

        match event {
            Event::Key(res) => {
                match res.code {
                    KeyCode::Char(c) => {
                        self.value.push(c);
                        Action::Internal
                    },
                    KeyCode::Backspace => {
                        self.value.pop();
                        Action::Internal
                    },
                    KeyCode::Enter => {
                        // run query
                        let q = self.value.clone();
                        self.value_stack.insert(0, self.value.clone());
                        self.stack_pos = 0;
                        self.value = String::new();
                        Action::RunQuery(q)
                    },
                    KeyCode::Down => {
                        if self.stack_pos > 0 {
                            self.stack_pos -= 1;
                            if self.stack_pos == 0 {
                                self.value = String::new()
                            } else {
                                self.value = self.value_stack[self.stack_pos - 1].clone();
                            }
                        }
                        Action::Internal
                    },
                    KeyCode::Up => {

                        if self.stack_pos < self.value_stack.len() {
                            self.stack_pos += 1;
                            self.value = self.value_stack[self.stack_pos - 1].clone();
                        }
                        Action::Internal
                    },
                    _ => {Action::Internal}
                }
            }
            Event::Mouse(res) => {
                Action::Internal
            }
            _ => {
                Action::Internal
            }
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
