use zeryon_core::lexer::Lexer;
use zeryon_core::parser::Parser;

fn main() {
    let code = r#"
        app MiApp {
            start: HomePage
        }

        page HomePage {
            layout: Column {
                text: "Hola"
                size: 32
            }
        }
    "#;

    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    println!("{:#?}", ast);
}