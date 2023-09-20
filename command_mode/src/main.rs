use command_mode::{copy::CopyCommand, cut::CutCommand, paste::PasteCommand, AppContext};
use cursive::{
    view::Nameable,
    views::{Dialog, EditView},
    Cursive,
};

fn main() {
    println!("Hello, world!");

    let mut app = cursive::default();

    app.set_user_data(AppContext::default());
    app.add_layer(
        Dialog::around(EditView::default().with_name("Editor"))
            .title("Type and use buttons")
            .button("Copy", |s| execute(s, CopyCommand::default()))
            .button("Cut", |s| execute(s, CutCommand::default()))
            .button("Paste", |s| execute(s, PasteCommand::default()))
            .button("Undo", undo)
            .button("Quit", |s| s.quit()),
    );
    app.run();
}

fn execute(app: &mut Cursive, mut command: impl command_mode::command::Command + 'static) {
    if command.execute(app) {
        app.with_user_data(|context: &mut AppContext| {
            context.history.push(Box::new(command));
        });
    }
}

fn undo(app: &mut Cursive) {
    let mut context = app.take_user_data::<AppContext>().unwrap();
    if let Some(mut command) = context.history.pop() {
        command.undo(app);
    }
    app.set_user_data(context);
}
