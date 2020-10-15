use crate::statements::create::CreateTableStatement;
use crate::statements::insert::InsertStatement;
use crate::statements::select::SelectStatement;

pub mod create;
pub mod compiler;
pub mod insert;
pub mod select;
pub mod scanner;


#[derive(Debug)]
pub enum Statement {
    Create(CreateTableStatement),
    Insert(InsertStatement),
    Select(SelectStatement),
}

