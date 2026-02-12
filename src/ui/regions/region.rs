use crate::ui::framebuffer::{Cell, FrameBuffer};
use crossterm::event::Event;

#[derive(PartialEq)]
pub enum RegionType {
    Input,
    Table,
    List
}

pub enum RegionData {
    Table(Vec<String>),
    List(String)
}

pub trait Region {
    fn build_inner_buffer(&self) -> Vec<Cell>;

    fn build_region_buffer(&self) -> Vec<Cell>;

    fn draw(&self, fb: &mut FrameBuffer);

    /// Called when an event happens on focused regions as long as the TUI event handler doesn't consume the event
    fn handle_event(&mut self, event: Event);

    fn _debug(&self);

    fn bounds_loc(&self, x: u16, y: u16) -> bool;

    fn set_focus(&mut self, focus: bool);

    fn _type(&self) -> RegionType;

    fn send_data(&mut self, data: RegionData);
}
