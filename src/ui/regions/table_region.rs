use crate::ui::event_action::Action;
use crate::ui::framebuffer::Cell;
use crate::ui::regions::region::{Region, RegionData, RegionType, ScrollTrack};
use crate::ui::tui::{Colour, Log};
use crate::utils::utils::{bounds_loc, micro_secs_now};
use crossterm::event::{Event, KeyModifiers, MouseEventKind};

pub struct TableRegion {
    pub name: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub border_colour: Colour,
    pub focused_border_colour: Colour,
    pub formatted_table: Vec<String>,
    pub focused: bool,
    ///stored as (last scroll time, scroll position)
    pub vertical_scroll: ScrollTrack,
    ///stored as (last scroll time, scroll position)
    pub horizontal_scroll: ScrollTrack,
}

impl TableRegion {
    const SCROLL_DEBOUNCE_MICRO: u128 = 30_000; // 100 milliseconds

    fn change_vertical_scroll(&mut self, change: isize, ignore_debounce: bool) {
        // the debounce is needed because multiple scroll events were firing per notch on the scroll wheel
        if (micro_secs_now() - self.vertical_scroll.0) < Self::SCROLL_DEBOUNCE_MICRO
            && !ignore_debounce
        {
            return;
        }

        if self.formatted_table.len() == 0 {
            return;
        }
        if self.formatted_table.len() < self.height as usize {
            return;
        }

        let new = self.vertical_scroll.1.cast_signed() + change;

        if new >= 0 && new <= (self.formatted_table.len() - self.height as usize + 2).cast_signed()
        // + 2 accounts for border weirdness
        {
            self.vertical_scroll = (micro_secs_now(), new.cast_unsigned());
        }
    }

    fn change_horizontal_scroll(&mut self, change: isize, ignore_debounce: bool) {
        if (micro_secs_now() - self.horizontal_scroll.0) < Self::SCROLL_DEBOUNCE_MICRO
            && !ignore_debounce
        {
            return;
        }

        if self.formatted_table.len() == 0 {
            return;
        }
        if self.formatted_table[0].len() < self.width as usize {
            return;
        }

        let new = self.horizontal_scroll.1.cast_signed() + change;

        if !(new < 0
            || new > (self.formatted_table[0].len() - self.width as usize + 2).cast_signed())
        {
            self.horizontal_scroll = (micro_secs_now(), new.cast_unsigned());
        }
    }
}

impl Region for TableRegion {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        let mut buffer = vec![
            Cell {
                char: ' ',
                colour: Colour::White,
                bold: false
            };
            ((self.width - 2) * (self.height - 2)) as usize
        ];

        for (y, row) in self.formatted_table.iter().enumerate() {
            if y < self.vertical_scroll.1 {
                continue;
            }
            if (y - self.vertical_scroll.1) >= (self.height - 2) as usize {
                break;
            }
            for (x, char) in row.chars().enumerate() {
                if x < self.horizontal_scroll.1 {
                    continue;
                }
                if (x - self.horizontal_scroll.1) >= (self.width - 2) as usize {
                    break;
                }
                buffer[(y - self.vertical_scroll.1) * (self.width - 2) as usize
                    + (x - self.horizontal_scroll.1)] = Cell {
                    char,
                    colour: Colour::White,
                    bold: false,
                }
            }
        }

        buffer
    }

    fn handle_event(&mut self, event: Event, _lb: &mut Vec<Log>) -> Action {
        match event {
            Event::Mouse(res) => {
                match res.kind {
                    MouseEventKind::ScrollUp => {
                        if self.bounds_coords(res.column, res.row) {
                            self.change_vertical_scroll(
                                -1,
                                res.modifiers.contains(KeyModifiers::CONTROL),
                            ) // holding control will ignore the debounce to allow faster scrolling
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if self.bounds_coords(res.column, res.row) {
                            self.change_vertical_scroll(
                                1,
                                res.modifiers.contains(KeyModifiers::CONTROL),
                            )
                        }
                    }
                    MouseEventKind::ScrollLeft => {
                        if self.bounds_coords(res.column, res.row) {
                            self.change_horizontal_scroll(
                                1,
                                res.modifiers.contains(KeyModifiers::CONTROL),
                            )
                        }
                    }
                    MouseEventKind::ScrollRight => {
                        if self.bounds_coords(res.column, res.row) {
                            self.change_horizontal_scroll(
                                -1,
                                res.modifiers.contains(KeyModifiers::CONTROL),
                            )
                        }
                    }
                    _ => {}
                }
                Action::Internal
            }
            _ => Action::Internal,
        }
    }

    fn bounds_coords(&self, x: u16, y: u16) -> bool {
        bounds_loc(self.x, self.y, self.width, self.height, x, y)
    }

    fn set_focus(&mut self, focus: bool) {
        self.focused = focus;
    }

    fn _type(&self) -> RegionType {
        RegionType::Table
    }

    fn _debug(&self) {
        println!(
            "width: {}, height: {}, x: {}, y: {}",
            self.width, self.height, self.x, self.y
        );
    }

    fn send_data(&mut self, data: RegionData) {
        match data {
            RegionData::Table(res) => {
                self.formatted_table = res;
                self.vertical_scroll = (0, 0);
                self.horizontal_scroll = (0, 0);
            }
            _ => {} // ignore non table data
        }
    }

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

    fn get_pos(&self) -> (u16, u16, i16) {
        (self.x, self.y, 0)
    }
    fn clear_buffer(&mut self) {
        self.formatted_table.clear()
    }
}
