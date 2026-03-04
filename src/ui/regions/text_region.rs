use crate::ui::event_action::Action;
use crate::ui::framebuffer::{Cell, FrameBuffer};
use crate::ui::regions::region::{Region, RegionData, RegionType};
use crate::ui::tui::{Colour, Log};
use crate::utils::utils::bounds_loc;
use crossterm::event::Event;
use std::cmp::max;

#[derive(Clone)]
pub enum TextAlign {
    Left,
    Right,
    Center,
}

#[derive(Clone)]
pub struct TextRegion {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub border_colour: Colour,
    pub focused_border_colour: Colour,
    pub focused: bool,
    pub text: Vec<String>,
    pub text_align: TextAlign,
}

impl Region for TextRegion {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        let mut buffer = vec![
            Cell {
                char: ' ',
                colour: Colour::White,
                bold: false
            };
            ((self.width - 2) * (self.height - 2)) as usize
        ];

        for (y, row) in self.text.iter().enumerate() {
            if y >= (self.height - 2) as usize {
                break;
            }
            let offset: usize = match self.text_align {
                TextAlign::Left => 0,
                TextAlign::Center => max(
                    ((self.width as usize - row.len()) as f32 / 2.0).floor() as usize,
                    0,
                ),
                TextAlign::Right => max((self.width as usize - row.len()) as usize, 0),
            };
            for (x, char) in row.chars().enumerate() {
                if x >= (self.width - 2) as usize {
                    break;
                }
                buffer[y * (self.width - 2) as usize + x + offset] = Cell {
                    char,
                    colour: Colour::White,
                    bold: false,
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
                        buffer.push(Cell {
                            char: '─',
                            colour: c,
                            bold: self.focused,
                        })
                    }
                } else if y == self.height - 1 {
                    if x == 0 {
                        buffer.push(Cell {
                            char: '╰',
                            colour: c,
                            bold: self.focused,
                        })
                    } else if x == self.width - 1 {
                        buffer.push(Cell {
                            char: '╯',
                            colour: c,
                            bold: self.focused,
                        })
                    } else {
                        buffer.push(Cell {
                            char: '─',
                            colour: c,
                            bold: self.focused,
                        })
                    }
                } else {
                    if x == 0 || x == self.width - 1 {
                        buffer.push(Cell {
                            char: '│',
                            colour: c,
                            bold: self.focused,
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

    fn handle_event(&mut self, _event: Event, _lb: &mut Vec<Log>) -> Action {
        Action::Internal
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
        RegionType::List
    }

    fn send_data(&mut self, _data: RegionData) {} //a text region is static

    fn set_geometry(&mut self, x: u16, y: u16, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.x = x;
        self.y = y;
    }
}
