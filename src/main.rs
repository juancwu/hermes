use std::io;

mod api;
mod app;
mod components;
mod parser;
mod tui;

fn main() -> io::Result<()> {
    let tokens = parser::parser::parse(String::from("metadata { name some-name_hey1}"));
    println!("{:?}", tokens);
    let mut terminal = tui::init()?;
    let app_result = app::App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
