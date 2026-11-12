use crate::app_context::AppContext;
use crate::ui::char;
use crate::ui::components::component::Component;
use crate::ui::framebuffer::Cell;
use crate::ui::tui::Colour;

pub enum SettingValue {
    /// (`value`)
    Bool(bool),
    /// (`value`, `increment/decrement amount`)
    Int(i64, i64),
}

impl SettingValue {
    pub fn get_height(&self) -> u16 {
        match self {
            SettingValue::Bool(_) => 3,
            SettingValue::Int(_, _) => 10,
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
    pub fn init(&mut self, value: SettingValue) {
        self.height = value.get_height();
        self.setting_value = value;
    }

    pub fn build_bool_type_buffer(&self, value: &bool) -> Vec<Cell> {
        let mut buffer: Vec<Cell> = Vec::with_capacity((self.width * self.height) as usize);

        // top row
        buffer.append(&mut vec![Cell::empty(); self.width as usize - 3]);
        buffer.append(&mut vec![
            Cell::new(char::TL_PIPE),
            Cell::new(char::HORIZONTAL_PIPE),
            Cell::new(char::TR_PIPE),
        ]);

        // name and check section
        let mut truncated = self.name.clone();
        truncated.truncate(self.width as usize - 6);
        if truncated.len() < self.name.len() {
            truncated += "...";
        }

        buffer.append(&mut Cell::from_string(&truncated, Colour::White));
        buffer.append(&mut vec![
            Cell::empty();
            self.width as usize - 3 - truncated.len()
        ]);

        buffer.append(&mut vec![
            Cell::new(char::BL_PIPE),
            Cell::new(if value.clone() {
                ' '
            } else {
                char::FILLED_SQUARE
            }),
            Cell::new(char::BR_PIPE),
        ]);

        // bottom row
        buffer.append(&mut vec![Cell::empty(); self.width as usize - 3]);
        buffer.append(&mut vec![
            Cell::new(char::BL_PIPE),
            Cell::new(char::HORIZONTAL_PIPE),
            Cell::new(char::BR_PIPE),
        ]);
        buffer
    }
    pub fn build_int_type_buffer(&self) -> Vec<Cell> {
        Vec::new()
    }
}

impl Component for Setting {
    fn build_inner_buffer(&self) -> Vec<Cell> {
        match &self.setting_value {
            SettingValue::Bool(val) => self.build_bool_type_buffer(val),
            SettingValue::Int(_, _) => self.build_int_type_buffer(),
        }
    }

    fn build_region_buffer(&self) -> Vec<Cell> {
        self.build_inner_buffer()
    }

    fn get_geometry(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}
