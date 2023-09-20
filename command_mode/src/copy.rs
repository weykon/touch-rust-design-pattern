use cursive::views::EditView;

#[derive(Default)]
pub struct CopyCommand;
impl crate::command::Command for CopyCommand {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool {
        let editor = app.find_name::<EditView>("Editor").unwrap();
        let mut context = app.take_user_data::<crate::AppContext>().unwrap();

        context.clipboard = editor.get_content().to_string();

        app.set_user_data(context);
        false
    }

    fn undo(&mut self, app: &mut cursive::Cursive) {
        todo!()
    }
}
