use crate::app_context::AppContext;
use crate::ui::components::component::Component;
use crate::ui::framebuffer::Cell;

pub enum SettingValue {
    Bool(bool),
}

impl SettingValue {
    pub fn get_height(&self) -> u16 {
        match self {
            SettingValue::Bool(_) => 10,
        }
    }
}

pub struct Setting {
    name: String,
    width: u16,
    height: u16,
    setting_value: SettingValue,
    on_change: Option<Box<dyn Fn(&mut AppContext)>>,
}

impl Setting {
    pub fn init(&mut self) {
        self.height = self.setting_value.get_height();
    }

    pub fn build_bool_type_buffer(&self) -> Vec<Cell> {
        Vec::new()
    }
}

impl Component for Setting {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        match &self.setting_value {
            SettingValue::Bool(val) => self.build_bool_type_buffer(),
        }
    }

    fn build_region_buffer(&self) -> Vec<Cell> {
        Vec::new()
    }

    fn get_geometry(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}
