use crate::ui::framebuffer::{Cell, FrameBuffer};
use crate::ui::regions::region::Region;
use crate::ui::tui::Colour;

pub struct InputRegion {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub border_colour: Colour,
    pub value: String, // vector of lines to be displayed
}

impl Region for InputRegion {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        todo!()
    }

    fn build_region_buffer(&self) -> Vec<Cell> {
        todo!()
    }

    fn draw(&self, fb: &mut FrameBuffer) {
        todo!()
    }
}