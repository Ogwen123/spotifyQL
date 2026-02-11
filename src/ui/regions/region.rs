use crate::ui::framebuffer::{Cell, FrameBuffer};

pub trait Region {
    fn build_inner_buffer(&self) -> Vec<Cell>;

    fn build_region_buffer(&self) -> Vec<Cell>;

    fn draw(&self, fb: &mut FrameBuffer);
}