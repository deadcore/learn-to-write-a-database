use std::borrow::Borrow;

#[derive(Debug)]
pub enum Literal {
    U32(u32),
    String(String),
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal)
}

#[derive(Debug)]
pub struct InsertStatement {
    table: String,
    values: Vec<Expression>,
}

impl InsertStatement {
    pub fn new(table: String, values: Vec<Expression>) -> Self {
        InsertStatement { table, values }
    }

    pub fn table_name(&self) -> &str {
        self.table.borrow()
    }

    pub fn values(&self) -> &[Expression] {
        self.values.borrow()
    }
}