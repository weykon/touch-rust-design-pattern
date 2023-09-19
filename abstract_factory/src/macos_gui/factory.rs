pub struct MacFactory;
use super::button::Button as MacButton;
use super::checkbox::Checkbox as MacCheckbox;
use abstract_factory::{Button, Checkbox, GuiFactory};
impl GuiFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}
