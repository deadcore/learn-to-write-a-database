use std::borrow::Borrow;

#[derive(Debug)]
pub enum Expression {
    Column(String),
    All,
}

#[derive(Debug)]
pub struct SelectStatement {
    item: Vec<Expression>,
    from: String,
}

impl SelectStatement {
    pub fn new(item: Vec<Expression>, from: String) -> Self {
        SelectStatement { item, from }
    }

    pub fn table_name(&self) -> &str {
        self.from.as_str()
    }

    pub fn expression(&self) -> &[Expression] {
        self.item.borrow()
    }
}