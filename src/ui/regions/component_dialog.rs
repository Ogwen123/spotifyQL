use crate::ui::char;
use crate::ui::components::component::Component;
use crate::ui::event_action::Action;
use crate::ui::framebuffer::Cell;
use crate::ui::framebuffer::FrameBuffer;
use crate::ui::regions::region::{Region, RegionData, RegionType, ScrollTrack};
use crate::ui::tui::{Colour, Log};
use crate::utils::utils::{bounds_coords, micro_secs_now};
use crossterm::event::{Event, KeyCode, KeyModifiers, MouseEventKind};

pub struct ComponentDialog {
    pub id: String,
    pub name: String,
    pub border_colour: Colour,
    pub show: bool,
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
    pub z_index: i16,
    pub components: Vec<Box<dyn Component>>,
    pub vertical_scroll: ScrollTrack,
}

impl ComponentDialog {
    const SCROLL_DEBOUNCE_MICRO: u128 = 30_000; // 30 milliseconds

    fn content_height(&self) -> u16 {
        self.components.iter().map(|x| x.get_geometry().1).sum()
    }

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

impl Region for ComponentDialog {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        let mut buffer: Vec<Cell> = Vec::new();

        for (index, component) in self.components.iter().enumerate() {
            buffer.append(&mut component.build_inner_buffer());

            if index != self.components.len() {
                buffer.append(&mut vec![
                    Cell {
                        char: char::HORIZONTAL_PIPE,
                        colour: self.border_colour.clone(),
                        bold: false
                    };
                    component.get_geometry().0 as usize
                ])
            }
        }

        buffer
    }

    fn handle_event(&mut self, event: Event, lb: &mut Vec<Log>) -> Action {
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
                    _ => {}
                }
                Action::Internal
            }
            _ => Action::Internal,
        }
    }

    fn bounds_coords(&self, x: u16, y: u16) -> bool {
        bounds_coords(self.x, self.y, self.width, self.height, x, y)
    }
    
    fn set_focus(&mut self, focus: bool) {
        self.show = focus;
    }

    fn is_focused(&self) -> bool {
        self.show
    }

    fn _type(&self) -> RegionType {
        RegionType::Dialog
    }

    fn is(&self, identifier: String) -> bool {
        identifier == self.id
    }

    fn _debug(&self) {
        todo!()
    }

    fn send_data(&mut self, data: RegionData) {
        todo!()
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
            self.show,
            &self.border_colour,
            &self.border_colour,
            &self.name,
        )
    }

    fn get_pos(&self) -> (u16, u16, i16) {
        (0, 0, self.z_index)
    }

    fn set_pos(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}
