use crate::ast::{
    AppDecl, ComponentDecl, Layout as LayoutNode, LayoutType, Literal, PageDecl, Program, Property,
    Declaration,
};
use crate::token::{Token, Token::*};
use std::string::String;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(mut lexer: crate::lexer::Lexer) -> Self {
        let tokens = lexer.tokenize();
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Program {
        let mut declarations = Vec::new();
        while !self.is_at_end() {
            if let Some(decl) = self.parse_declaration() {
                declarations.push(decl);
            }
        }
        Program { declarations }
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        match self.peek() {
            App => Some(Declaration::App(self.parse_app())),
            Page => Some(Declaration::Page(self.parse_page())),
            Component => Some(Declaration::Component(self.parse_component())),
            _ => {
                self.advance();
                None
            }
        }
    }

    fn parse_app(&mut self) -> AppDecl {
        self.consume(App);
        let name = self.consume_identifier();
        self.consume(LeftBrace);
        self.consume(Start);
        self.consume(Colon);
        let start_page = self.consume_identifier();
        self.consume(RightBrace);
        AppDecl { name, start_page }
    }

    fn parse_page(&mut self) -> PageDecl {
        self.consume(Page);
        let name = self.consume_identifier();
        self.consume(LeftBrace);
        let layout = self.parse_layout();
        self.consume(RightBrace);
        PageDecl { name, layout }
    }

    fn parse_component(&mut self) -> ComponentDecl {
        self.consume(Component);
        let name = self.consume_identifier();
        self.consume(LeftBrace);
        let layout = self.parse_layout();
        self.consume(RightBrace);
        ComponentDecl { name, layout }
    }

    fn parse_layout(&mut self) -> LayoutNode {
        self.consume(Layout);
        self.consume(Colon);
        let layout_type = match self.advance() {
            Column => LayoutType::Column,
            Row => LayoutType::Row,
            Stack => LayoutType::Stack,
            Container => LayoutType::Container,
            _ => panic!("Unexpected layout type"),
        };
        self.consume(LeftBrace);
        let mut properties = Vec::new();
        while !self.check(RightBrace) && !self.is_at_end() {
            if let Some(prop) = self.parse_property() {
                properties.push(prop);
            }
        }
        self.consume(RightBrace);
        LayoutNode {
            layout_type,
            properties,
        }
    }

    fn parse_property(&mut self) -> Option<Property> {
        let key = self.consume_identifier();
        self.consume(Colon);
        let value = match self.advance() {
            String(s) => Literal::String(s),
            Number(n) => Literal::Number(n),
            Boolean(b) => Literal::Boolean(b),
            Identifier(i) => Literal::Identifier(i),
            _ => return None,
        };
        Some(Property { key, value })
    }

    // helpers
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Eof)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Eof)
    }

    fn check(&self, token: Token) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(&token)
    }

    fn consume(&mut self, expected: Token) {
        if self.check(expected.clone()) {
            self.advance();
        } else {
            panic!("Expected {:?}, found {:?}", expected, self.peek());
        }
    }

    fn consume_identifier(&mut self) -> String {
        match self.advance() {
            Identifier(s) => s,
            _ => panic!("Expected identifier"),
        }
    }
}