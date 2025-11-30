#[derive(Debug, PartialEq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub enum Declaration {
    App(AppDecl),
    Page(PageDecl),
    Component(ComponentDecl),
}

#[derive(Debug, PartialEq)]
pub struct AppDecl {
    pub name: String,
    pub start_page: String,
}

#[derive(Debug, PartialEq)]
pub struct PageDecl {
    pub name: String,
    pub layout: Layout,
}

#[derive(Debug, PartialEq)]
pub struct ComponentDecl {
    pub name: String,
    pub layout: Layout,
}

#[derive(Debug, PartialEq)]
pub struct Layout {
    pub layout_type: LayoutType,
    pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq)]
pub enum LayoutType {
    Column,
    Row,
    Stack,
    Container,
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub key: String,
    pub value: Literal,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
}