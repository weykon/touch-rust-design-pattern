pub mod command;
pub mod copy;
pub mod cut;
pub mod paste;

#[derive(Default)]
pub struct AppContext {
    pub clipboard: String,
    pub history: Vec<Box<dyn command::Command>>,
}
