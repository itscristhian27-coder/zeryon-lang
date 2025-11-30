use zeryon_core::lexer::Lexer;

fn main() {
    let code = r#"
        app MiApp {
            start: HomePage
        }
    "#;

    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}