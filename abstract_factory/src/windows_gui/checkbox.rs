pub struct Checkbox;
impl abstract_factory::Checkbox for Checkbox {
    fn switch(&self) {
        println!("Windows Checkbox switched");
    }
}
