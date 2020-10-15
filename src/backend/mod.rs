use crate::Result;
use crate::statements::create::CreateTableStatement;
use crate::statements::insert::InsertStatement;
use crate::statements::select::SelectStatement;

pub mod memory;

#[derive(Debug, PartialEq, Eq)]
pub struct QueryResults {
    cells: Vec<Vec<Cell>>
}

impl QueryResults {
    pub fn new(cells: Vec<Vec<Cell>>) -> Self {
        QueryResults { cells }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    U32(u32),
    String(String),
}

pub trait Backend {
    fn create_table(&mut self, stmt: &CreateTableStatement) -> Result<()>;
    fn insert(&mut self, stmt: &InsertStatement) -> Result<()>;
    fn select(&mut self, stmt: &SelectStatement) -> Result<QueryResults>;
}