pub struct Button;
impl abstract_factory::Button for Button {
    fn press(&self) {
        println!("MacOS button pressed");
    }
}
