use cursive::{views::EditView, Cursive};
pub struct CopyCommand;
impl super::command::Command  for CopyCommand {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool {
        todo!()
    }

    fn undo(&mut self, app: &mut cursive::Cursive) {
        todo!()
    }
}
