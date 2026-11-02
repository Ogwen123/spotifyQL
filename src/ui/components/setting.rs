use crate::ui::components::component::Component;

pub enum SettingType {
    Bool
}

pub struct Setting {
    name: String,
    setting_type: SettingType,
    on_change: Option<Box<dyn Fn()>>,
}

//impl Component for Setting {}