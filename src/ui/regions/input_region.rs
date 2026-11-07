use crate::ui::event_action::Action;
use crate::ui::framebuffer::Cell;
use crate::ui::regions::region::{Region, RegionData, RegionType};
use crate::ui::tui::{Colour, Log};
use crate::utils::utils::bounds_loc;
use crossterm::event::{Event, KeyCode};

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
                    bold: false,
                }
            }
        } else {
            for (index, char) in self.value.chars().enumerate() {
                buffer[index] = Cell {
                    char,
                    colour: Colour::White,
                    bold: false,
                }
            }
        }

        buffer
    }

    fn handle_event(&mut self, event: Event, _lb: &mut Vec<Log>) -> Action {
        if !self.focused {
            return Action::Internal;
        }

        match event {
            Event::Key(res) => {
                match res.code {
                    KeyCode::Char(c) => {
                        self.value.push(c);
                        Action::Internal
                    }
                    KeyCode::Backspace => {
                        self.value.pop();
                        Action::Internal
                    }
                    KeyCode::Enter => {
                        // run query
                        let q = self.value.clone();
                        self.value_stack.insert(0, self.value.clone());
                        self.stack_pos = 0;
                        self.value = String::new();
                        Action::RunQuery(q)
                    }
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
                    }
                    KeyCode::Up => {
                        if self.stack_pos < self.value_stack.len() {
                            self.stack_pos += 1;
                            self.value = self.value_stack[self.stack_pos - 1].clone();
                        }
                        Action::Internal
                    }
                    _ => Action::Internal,
                }
            }
            Event::Mouse(_res) => Action::Internal,
            _ => Action::Internal,
        }
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

    fn _debug(&self) {
        println!(
            "width: {}, height: {}, x: {}, y: {}",
            self.width, self.height, self.x, self.y
        );
    }

    fn send_data(&mut self, _data: RegionData) {}

    fn set_geometry(&mut self, x: u16, y: u16, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.x = x;
        self.y = y;
    }

    fn get_border_data(&self) -> (u16, u16, bool, &Colour, &Colour, &String) {
        (
            self.width,
            self.height,
            self.focused,
            &self.border_colour,
            &self.focused_border_colour,
            &self.name,
        )
    }

    fn get_pos(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    fn clear_buffer(&mut self) {
        self.value.clear();
    }
}
