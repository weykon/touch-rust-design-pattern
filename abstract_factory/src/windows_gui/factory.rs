use super::button::Button as WindowsButton;
use super::checkbox::Checkbox as WindowsCheckbox;
use abstract_factory::{Button, Checkbox, GuiFactory};

pub struct WindowsFactory;
impl GuiFactory for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}
