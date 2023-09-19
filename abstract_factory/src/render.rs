use abstract_factory::GuiFactory;

pub fn render(factory: impl GuiFactory) {
    let button1 = factory.create_button();
    let button2 = factory.create_button();
    let checkbox1 = factory.create_checkbox();
    let checkbox2 = factory.create_checkbox();

    use abstract_factory::{Button, Checkbox};

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}
