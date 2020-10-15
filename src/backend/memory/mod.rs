use std::collections::HashMap;

use crate::backend::{Backend, Cell, QueryResults};
use crate::Result;
use crate::statements::{insert, select};
use crate::statements::create::{CreateTableStatement, DataType};
use crate::statements::insert::{InsertStatement, Literal};
use crate::statements::select::SelectStatement;

#[derive(Debug, PartialEq, Eq)]
pub enum ColumnTypes {
    Int32,
    String,
}

pub enum MemoryCell {
    U32(u32),
    String(String),
}

pub struct Rows {
    stride: usize,
    data: Vec<MemoryCell>,
}

impl Extend<MemoryCell> for Rows {
    fn extend<T: IntoIterator<Item=MemoryCell>>(&mut self, iter: T) {
        self.data.extend(iter)
    }
}

impl Rows {
    pub fn new(stride: usize, data: Vec<MemoryCell>) -> Self {
        Rows { stride, data }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Column {
    name: String,
    _type: ColumnTypes,
}

impl Column {
    pub fn new(name: String, _type: ColumnTypes) -> Self {
        Column { name, _type }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn column_type(&self) -> &ColumnTypes {
        &self._type
    }
}

pub struct Table {
    columns: Vec<Column>,
    rows: Rows,
}

impl Table {
    pub fn columns(&self) -> &[Column] {
        self.columns.as_ref()
    }

    pub fn rows(&self) -> &Rows {
        &self.rows
    }
}

pub struct InMemoryBackend {
    tables: HashMap<String, Table>
}

impl InMemoryBackend {
    pub fn new(tables: HashMap<String, Table>) -> Self {
        InMemoryBackend { tables }
    }
}

impl Backend for InMemoryBackend {
    fn create_table(&mut self, stmt: &CreateTableStatement) -> Result<()> {
        if self.tables.contains_key(stmt.table_name()) {
            return Err(format!("Table {:#?} already exists", stmt.table_name()).into());
        }

        let columns = stmt.columns();

        let metadata = columns.iter().map(|column| {
            Column::new(
                column.name().to_owned(),
                match column.data_type() {
                    DataType::Int32 => ColumnTypes::Int32,
                    DataType::String => ColumnTypes::String,
                })
        }).collect::<Vec<Column>>();

        let table = Table {
            columns: metadata,
            rows: Rows::new(
                columns.len(),
                Vec::new(),
            ),
        };

        self.tables.insert(stmt.table_name().to_owned(), table);

        Ok(())
    }

    fn insert(&mut self, stmt: &InsertStatement) -> Result<()> {
        return match self.tables.get_mut(stmt.table_name()) {
            None => Err(format!("Table {:#?} not found", stmt.table_name()).into()),
            Some(table) => {
                let values = stmt.values();
                if values.len() != table.columns().len() {
                    return Err(format!("Incorrect number of column. Expected {:?} but found {:?}", table.columns.len(), values.len()).into());
                }

                let mut row: Vec<MemoryCell> = Vec::with_capacity(values.len());

                for (expression, column_type) in stmt.values().iter().zip(table.columns()) {
                    match expression {
                        insert::Expression::Literal(literal) => {
                            match literal {
                                Literal::U32(value) => {
                                    if *column_type.column_type() != ColumnTypes::Int32 {
                                        return Err(format!("Expected Int32 but got {:?}", column_type).into());
                                    }
                                    row.push(MemoryCell::U32(*value))
                                }
                                Literal::String(value) => {
                                    if *column_type.column_type() != ColumnTypes::String {
                                        return Err(format!("Expected String but got {:?}", column_type).into());
                                    }
                                    row.push(MemoryCell::String(value.to_string()))
                                }
                            }
                        }
                    }
                }

                table.rows.extend(row);

                Ok(())
            }
        };
    }

    fn select(&mut self, stmt: &SelectStatement) -> Result<QueryResults> {
        return match self.tables.get_mut(stmt.table_name()) {
            None => Err(format!("Table {:#?} not found", stmt.table_name()).into()),
            Some(table) => {
                let mut results = Vec::new();
                let mut indexes = Vec::new();

                for expression in stmt.expression() {
                    let x = match expression {
                        select::Expression::Column(column_name) => {
                            let maybe_column = table.columns.iter().enumerate().find(|(index, column)| column.name() == column_name);
                            match maybe_column {
                                Some((index, column)) => {
                                    indexes.push(index)
                                }
                                None => {
                                    return Err(format!("Column {:?} is not found in the Table {:?}", column_name, stmt.table_name()).into());
                                }
                            }
                        }
                        select::Expression::All => {
                            indexes.extend(
                                table.columns.iter().enumerate().map(|(index, column)| index)
                            );
                        }
                    };
                };

                let rows = table.rows();
                let data = &table.rows.data;

                for row in (0..rows.data.len()).step_by(rows.stride) {
                    let row = indexes.iter()
                        .map(|index| &data[index + row])
                        .map(|memory_cell| {
                            match memory_cell {
                                MemoryCell::U32(value) => Cell::U32(*value),
                                MemoryCell::String(value) => Cell::String(value.to_string()),
                            }
                        })
                        .collect::<Vec<Cell>>();
                    results.push(row)
                }

                let query_results = QueryResults::new(results);

                return Ok(query_results);
            }
        };
    }
}