// use std::io;

mod api;
mod app;
mod components;
mod lexer;
mod parser;
mod transition_table;
mod tui;

// fn main() -> io::Result<()> {
//     let tokens = parser::parser::parse("metadata { name some-name_hey1}");
//     println!("{:?}", tokens);
//     let mut terminal = tui::init()?;
//     let app_result = app::App::default().run(&mut terminal);
//     tui::restore()?;
//     app_result
// }

fn main() {}
