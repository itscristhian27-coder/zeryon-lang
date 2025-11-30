#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Palabras clave
    App,
    Page,
    Component,
    Import,
    Start,
    Layout,
    Column,
    Row,
    Stack,
    Container,
    Text,
    Image,
    Button,
    Fetch,
    State,
    Style,
    Font,
    Color,
    Asset,
    Export,
    Function,
    True,
    False,

    // Símbolos
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    Dot,
    Arrow,

    // Literales
    Identifier(String),
    String(String),
    Number(f64),
    Boolean(bool),

    // Fin de archivo
    Eof,
}