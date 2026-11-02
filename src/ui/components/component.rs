use crate::ui::framebuffer::Cell;

pub trait Component {
    fn build_inner_buffer(&self) -> Vec<Cell>;
    fn build_region_buffer(&self) -> Vec<Cell>;
}
