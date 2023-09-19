pub struct MacFactory;
impl GuiFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}
