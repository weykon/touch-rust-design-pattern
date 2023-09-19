pub struct Checkbox;
impl abstract_factory::Checkbox for Checkbox {
    fn switch(&self) {
        println!("MacOS Checkbox switched");
    }
}
