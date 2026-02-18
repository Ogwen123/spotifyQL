use std::cmp::max;
use crate::ui::framebuffer::{Cell, FrameBuffer};
use crate::ui::regions::region::{Region, RegionData, RegionType, REGION_NAME_PADDING};
use crate::ui::tui::{Colour, Log, TUI};
use crossterm::event::{Event, KeyModifiers, MouseEventKind};
use crate::app_context::AppContext;
use crate::ui::event_action::Action;
use crate::utils::utils::{bounds_loc, micro_secs_now};

#[derive(Clone)]
pub struct ListRegion {
    pub name: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub border_colour: Colour,
    pub focused_border_colour: Colour,
    pub data: Vec<Log>, // vector of lines to be displayed
    pub longest_log: usize,
    pub focused: bool,
    ///stored as (last scroll time, scroll position)
    pub vertical_scroll: (u128, usize),
    ///stored as (last scroll time, scroll position)
    pub horizontal_scroll: (u128, usize),
}

impl ListRegion {
    const SCROLL_DEBOUNCE_MICRO: u128 = 50_000; // 100 milliseconds

    fn change_vertical_scroll(&mut self, change: isize, ignore_debounce: bool) {

        // the debounce is needed because multiple scroll events were firing per notch on the scroll wheel
        if (micro_secs_now() - self.vertical_scroll.0) < Self::SCROLL_DEBOUNCE_MICRO && !ignore_debounce {return}

        if self.data.len() == 0 {
            return;
        }
        if self.data.len() < self.height as usize {
            return;
        }

        let new = self.vertical_scroll.1.cast_signed() + change;

        if new >= 0 && new <= (self.data.len() - self.height as usize + 2).cast_signed() // + 2 accounts for border weirdness
        {
            self.vertical_scroll = (micro_secs_now(), new.cast_unsigned());
        }
    }

    fn change_horizontal_scroll(&mut self, change: isize, ignore_debounce: bool) {
        if (micro_secs_now() - self.horizontal_scroll.0) < Self::SCROLL_DEBOUNCE_MICRO && !ignore_debounce {return}

        if self.longest_log == 0 {
            return;
        }
        if self.longest_log < self.width as usize {
            return;
        }

        let new = self.horizontal_scroll.1.cast_signed() + change;

        if !(new < 0
            || new > (self.longest_log - self.width as usize + 2).cast_signed())
        {
            self.horizontal_scroll = (micro_secs_now(), new.cast_unsigned());
        }
    }
}

impl Region for ListRegion {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        let mut buffer = vec![
            Cell {
                char: ' ',
                colour: Colour::White,
                bold: false
            };
            ((self.width - 2) * (self.height - 2)) as usize
        ];

        for (y, row) in self.data.iter().enumerate() {
            if y < self.vertical_scroll.1 {continue}
            if (y - self.vertical_scroll.1) >= (self.height - 2) as usize {
                break;
            }
            for (x, char) in row.content.chars().enumerate() {
                if x < self.horizontal_scroll.1 {continue}
                if (x - self.horizontal_scroll.1) >= (self.width - 2) as usize {
                    break;
                }
                buffer[(y - self.vertical_scroll.1) * (self.width - 2) as usize + (x - self.horizontal_scroll.1)] = Cell {
                    char,
                    colour: row.severity.colour(),
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
            let local_y = (loc as f64 / fb.width as f64).floor() as u16;
            let local_x = loc as u16 % fb.width;

            fb.put(local_x + self.x, local_y + self.y, cell)
        }
    }

    fn handle_event(&mut self, event: Event, lb: &mut Vec<Log>) -> Action {
        match event {
            Event::Mouse(res) => {
                match res.kind {
                    MouseEventKind::ScrollUp => {
                        if self.bounds_loc(res.column, res.row) {
                            self.change_vertical_scroll(-1, res.modifiers.contains(KeyModifiers::CONTROL))
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if self.bounds_loc(res.column, res.row) {
                            self.change_vertical_scroll(1, res.modifiers.contains(KeyModifiers::CONTROL))
                        }
                    }
                    MouseEventKind::ScrollLeft => {
                        if self.bounds_loc(res.column, res.row) {
                            self.change_horizontal_scroll(1, res.modifiers.contains(KeyModifiers::CONTROL))
                        }
                    }
                    MouseEventKind::ScrollRight => {
                        if self.bounds_loc(res.column, res.row) {
                            self.change_horizontal_scroll(-1, res.modifiers.contains(KeyModifiers::CONTROL))
                        }
                    }
                    _ => {}
                }
                Action::Internal
            }
            _ => Action::Internal,
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
        RegionType::List
    }

    fn send_data(&mut self, mut data: RegionData) {
        match data {
            RegionData::List(ref mut res) => {
                for i in res.iter() {
                    if i.content.len() > self.longest_log {
                        self.longest_log = i.content.len()
                    }
                }
                self.data.append(res);
                // move scroll to the bottom of the logs
                self.vertical_scroll = (self.vertical_scroll.0, max(self.data.len() as isize - self.height as isize + 2, 0) as usize)
            },
            _ => {}
        }
    }

    fn set_geometry(&mut self, x: u16, y: u16, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.x = x;
        self.y = y;
    }
}
