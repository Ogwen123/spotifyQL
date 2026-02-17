use crate::ui::framebuffer::{Cell, FrameBuffer};
use crossterm::event::Event;
use crate::ui::event_action::Action;
use crate::ui::tui::{Log};

pub static REGION_NAME_PADDING: u16 = 2;

#[derive(PartialEq)]
pub enum RegionType {
    Input,
    Table,
    List
}

pub enum RegionData {
    Table(Vec<String>),
    List(Vec<Log>)
}

pub trait Region {
    fn build_inner_buffer(&self) -> Vec<Cell>;

    fn build_region_buffer(&self) -> Vec<Cell>;

    fn draw(&self, fb: &mut FrameBuffer);

    /// Called when an event happens on focused regions as long as the TUI event handler doesn't consume the event
    fn handle_event(&mut self, event: Event, lb: &mut Vec<Log>) -> Action;

    fn _debug(&self);

    fn bounds_loc(&self, x: u16, y: u16) -> bool;

    fn set_focus(&mut self, focus: bool);

    fn _type(&self) -> RegionType;

    fn send_data(&mut self, data: RegionData);
}
