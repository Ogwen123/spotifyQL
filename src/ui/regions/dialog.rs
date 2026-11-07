use crate::ui::components::component::Component;
use crate::ui::event_action::Action;
use crate::ui::framebuffer::Cell;
use crate::ui::regions::region::{Region, RegionData, RegionType, ScrollTrack};
use crate::ui::tui::{Colour, Log};
use crate::utils::utils::micro_secs_now;
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind};

pub struct Dialog {
    pub name: String,
    pub z_index: isize,
    pub height: usize,
    pub components: Vec<Box<dyn Component>>,
    pub vertical_scroll: ScrollTrack,
}

impl Dialog {
    const SCROLL_DEBOUNCE_MICRO: u128 = 30_000; // 30 milliseconds

    fn change_vertical_scroll(&mut self, change: isize, ignore_debounce: bool) {
        // the debounce is needed because multiple scroll events were firing per notch on the scroll wheel
        if (micro_secs_now() - self.vertical_scroll.0) < Self::SCROLL_DEBOUNCE_MICRO
            && !ignore_debounce
        {
            return;
        }

        let new = self.vertical_scroll.1.cast_signed() + change;

        if new >= 0 && new <= (self.height as usize + 2).cast_signed()
        // + 2 accounts for border weirdness
        {
            self.vertical_scroll = (micro_secs_now(), new.cast_unsigned());
        }
    }
}

impl Region for Dialog {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        todo!()
    }

    fn handle_event(&mut self, event: Event, lb: &mut Vec<Log>) -> Action {
        match event {
            Event::Mouse(res) => {
                match res.kind {
                    MouseEventKind::ScrollUp => {
                        if self.bounds_loc(res.column, res.row) {
                            self.change_vertical_scroll(
                                -1,
                                res.modifiers.contains(KeyModifiers::CONTROL),
                            ) // holding control will ignore the debounce to allow faster scrolling
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if self.bounds_loc(res.column, res.row) {
                            self.change_vertical_scroll(
                                1,
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

    fn bounds_loc(&self, x: u16, y: u16) -> bool {
        todo!()
    }

    fn set_focus(&mut self, focus: bool) {
        todo!()
    }

    fn _type(&self) -> RegionType {
        todo!()
    }

    fn _debug(&self) {
        todo!()
    }

    fn send_data(&mut self, data: RegionData) {
        todo!()
    }

    fn set_geometry(&mut self, x: u16, y: u16, width: u16, height: u16) {
        todo!()
    }

    fn get_border_data(&self) -> (u16, u16, bool, &Colour, &Colour, &String) {
        todo!()
    }

    fn get_pos(&self) -> (u16, u16) {
        todo!()
    }
}
