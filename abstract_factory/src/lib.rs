pub trait Button {
    fn press(&self);
}
pub trait Checkbox {
    fn switch(&self);
}

pub trait GuiFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}
