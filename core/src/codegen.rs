use crate::ast::*;
use std::collections::HashMap;

pub struct CodeGen;

impl CodeGen {
    pub fn new() -> Self {
        CodeGen
    }

    pub fn generate(&self, program: &Program) -> String {
        let mut output = String::new();
        output.push_str("{\n");
        for decl in &program.declarations {
            match decl {
                Declaration::App(app) => {
                    output.push_str(&format!("  \"app\": {{\n    \"name\": \"{}\",\n    \"start\": \"{}\"\n  }},\n", app.name, app.start_page));
                }
                Declaration::Page(page) => {
                    output.push_str(&format!("  \"page\": {{\n    \"name\": \"{}\",\n    \"layout\": {}\n  }},\n", page.name, self.gen_layout(&page.layout)));
                }
                Declaration::Component(comp) => {
                    output.push_str(&format!("  \"component\": {{\n    \"name\": \"{}\",\n    \"layout\": {}\n  }},\n", comp.name, self.gen_layout(&comp.layout)));
                }
            }
        }
        output.push_str("}\n");
        output
    }

    fn gen_layout(&self, layout: &Layout) -> String {
        let mut map = HashMap::new();
        map.insert("type", format!("{:?}", layout.layout_type));
        for prop in &layout.properties {
            map.insert(&prop.key, format!("{:?}", prop.value));
        }
        format!("{:?}", map)
    }
}