use crate::ui::char;
use crate::ui::event_action::Action;
use crate::ui::framebuffer::{Cell, FrameBuffer};
use crate::ui::tui::{Colour, Log};
use crossterm::event::Event;

pub static REGION_NAME_PADDING: u16 = 2;

pub type ScrollTrack = (u128, usize);

#[derive(PartialEq)]
pub enum RegionType {
    Input,
    Table,
    List,
    Dialog,
}

pub enum RegionData {
    Table(Vec<String>),
    List(Vec<Log>),
}

pub trait Region {
    fn build_inner_buffer(&self) -> Vec<Cell>;

    fn build_region_buffer(&self) -> Vec<Cell> {
        let mut buffer: Vec<Cell> = Vec::new();
        let inner_buffer = self.build_inner_buffer();

        let (width, height, focused, border_colour, focused_border_colour, name) =
            self.get_border_data();

        let name_chars = name.chars().collect::<Vec<char>>();

        for y in 0..height {
            for x in 0..width {
                let c = if focused {
                    focused_border_colour.clone()
                } else {
                    border_colour.clone()
                };

                if y == 0 {
                    if x == 0 {
                        buffer.push(Cell {
                            char: char::TL_PIPE,
                            colour: c,
                            bold: focused,
                        })
                    } else if x == width - 1 {
                        buffer.push(Cell {
                            char: char::TR_PIPE,
                            colour: c,
                            bold: focused,
                        })
                    } else {
                        let char = if x >= REGION_NAME_PADDING
                            && x - REGION_NAME_PADDING < name_chars.len() as u16
                        {
                            name_chars[(x - REGION_NAME_PADDING) as usize]
                        } else {
                            char::HORIZONTAL_PIPE
                        };

                        buffer.push(Cell {
                            char,
                            colour: c,
                            bold: focused,
                        })
                    }
                } else if y == height - 1 {
                    if x == 0 {
                        buffer.push(Cell {
                            char: char::BL_PIPE,
                            colour: c,
                            bold: focused,
                        })
                    } else if x == width - 1 {
                        buffer.push(Cell {
                            char: char::BR_PIPE,
                            colour: c,
                            bold: focused,
                        })
                    } else {
                        buffer.push(Cell {
                            char: char::HORIZONTAL_PIPE,
                            colour: c,
                            bold: focused,
                        })
                    }
                } else {
                    if x == 0 || x == width - 1 {
                        buffer.push(Cell {
                            char: char::VERTICAL_PIPE,
                            colour: c,
                            bold: focused,
                        })
                    } else {
                        buffer
                            .push(inner_buffer[((y - 1) * (width - 2) + (x - 1)) as usize].clone())
                    }
                }
            }
        }

        buffer
    }

    fn draw(&self, fb: &mut FrameBuffer) {
        let content = self.build_region_buffer();

        let (x, y) = self.get_pos();

        for (loc, cell) in content.into_iter().enumerate() {
            let local_y = (loc as f64 / fb.width as f64).floor() as u16;
            let local_x = loc as u16 % fb.width;

            fb.put(local_x + x, local_y + y, cell)
        }
    }

    /// Called when an event happens on focused regions as long as the TUI event handler doesn't consume the event
    fn handle_event(&mut self, event: Event, lb: &mut Vec<Log>) -> Action;
    fn bounds_loc(&self, x: u16, y: u16) -> bool;
    fn set_focus(&mut self, focus: bool);
    fn _type(&self) -> RegionType;
    fn _debug(&self);
    fn send_data(&mut self, data: RegionData);
    fn set_geometry(&mut self, x: u16, y: u16, width: u16, height: u16);
    fn get_border_data(&self) -> (u16, u16, bool, &Colour, &Colour, &String);
    fn get_pos(&self) -> (u16, u16);
    fn clear_buffer(&mut self) {}
}
