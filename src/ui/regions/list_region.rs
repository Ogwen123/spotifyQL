use crate::ui::framebuffer::{Cell, FrameBuffer};
use crate::ui::regions::region::Region;
use crate::ui::tui::Colour;
use crossterm::event::Event;
use crate::utils::utils::bounds_loc;

pub struct ListRegion {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub border_colour: Colour,
    pub focused_border_colour: Colour,
    pub data: Vec<String>, // vector of lines to be displayed
    pub focused: bool,
}

impl Region for ListRegion {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        vec![
            Cell {
                char: ' ',
                colour: Colour::White,
            };
            ((self.width - 2) * (self.height - 2)) as usize
        ]
    }

    /// Make a border of the given colour and fill with inner buffer (buffer length is 0)
    fn build_region_buffer(&self) -> Vec<Cell> {
        let mut buffer: Vec<Cell> = Vec::new();
        let inner_buffer = self.build_inner_buffer();

        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.border_colour.clone();

                if y == 0 {
                    if x == 0 {
                        buffer.push(Cell {
                            char: '╭',
                            colour: c,
                        })
                    } else if x == self.width - 1 {
                        buffer.push(Cell {
                            char: '╮',
                            colour: c,
                        })
                    } else {
                        buffer.push(Cell {
                            char: '─',
                            colour: c,
                        })
                    }
                } else if y == self.height - 1 {
                    if x == 0 {
                        buffer.push(Cell {
                            char: '╰',
                            colour: c,
                        })
                    } else if x == self.width - 1 {
                        buffer.push(Cell {
                            char: '╯',
                            colour: c,
                        })
                    } else {
                        buffer.push(Cell {
                            char: '─',
                            colour: c,
                        })
                    }
                } else {
                    if x == 0 || x == self.width - 1 {
                        buffer.push(Cell {
                            char: '│',
                            colour: c,
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
            let local_y = (loc as f64 / fb.width as f64).floor() as u16;
            let local_x = loc as u16 % fb.width;

            fb.put(local_x + self.x, local_y + self.y, cell)
        }
    }

    fn handle_event(&self, event: Event) {
        todo!()
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
}
