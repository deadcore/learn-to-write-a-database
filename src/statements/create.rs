use std::borrow::Borrow;

#[derive(Debug)]
pub enum DataType {
    Int32,
    String,
}

#[derive(Debug)]
pub struct ColumnDefinition {
    name: String,
    data_type: DataType,
}

impl ColumnDefinition {
    pub fn new(name: String, data_type: DataType) -> Self {
        ColumnDefinition { name, data_type }
    }
    pub fn name(&self) -> &str {
        self.name.borrow()
    }
    pub fn data_type(&self) -> &DataType {
        self.data_type.borrow()
    }
}

#[derive(Debug)]
pub struct CreateTableStatement {
    name: String,
    columns: Vec<ColumnDefinition>,
}

impl CreateTableStatement {
    pub fn new(name: String, columns: Vec<ColumnDefinition>) -> Self {
        CreateTableStatement { name, columns }
    }

    pub fn table_name(&self) -> &str {
        self.name.borrow()
    }

    pub fn columns(&self) -> &[ColumnDefinition] {
        self.columns.borrow()
    }
}
