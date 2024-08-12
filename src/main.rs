use std::io;

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

fn main() {
    // let mut lexer = parse::lexer::Lexer::new(
    //     r#"
    // collection::C {
    //     name "my collection"
    //     include "."
    //     environment 0 dev
    // }
    //
    // environment::dev {
    //     URL "/url"
    // }
    // "#,
    // );
    // let mut parser = parse::collection::CollectionParser::default();
    // let collection = parser.parse(&mut lexer);
    // println!("{:?}", collection);
    // let tokens = parser::parse::parse(
    //     r#"
    // metadata::m {
    //     name 0 "some value and escaped \n \""
    //     some-other2123_?dksj
    // }
    // "#,
    // );
    // println!("{:?}", tokens);
    // let mut terminal = tui::init()?;
    // let app_result = app::App::default().run(&mut terminal);
    // tui::restore()?;
    // app_result
    //
    let input = r#"
        body.json::json {
            value 1 _"{
                "name": {
                    "first": "Juan",
                    "last": "Wu"
                }
            }"_
        }
    "#;
    parser::parse(input);
}
