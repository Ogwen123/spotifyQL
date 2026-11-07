use crate::app_context::AppContext;
use crate::ui::components::component::Component;
use crate::ui::framebuffer::Cell;

pub enum SettingType {
    Bool,
}

pub struct Setting {
    name: String,
    width: u16,
    height: u16,
    setting_type: SettingType,
    on_change: Option<Box<dyn Fn(&mut AppContext)>>,
}

impl Setting {
    pub fn init(&mut self) {
        self.height = self.get_height();
    }

    pub fn get_height(&self) -> u16 {
        match self.setting_type {
            SettingType::Bool => 10,
        }
    }

    pub fn build_bool_type_buffer(&self) -> Vec<Cell> {
        Vec::new()
    }
}

impl Component for Setting {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        match &self.setting_type {
            SettingType::Bool => self.build_bool_type_buffer(),
        }
    }

    fn build_region_buffer(&self) -> Vec<Cell> {
        Vec::new()
    }
}
