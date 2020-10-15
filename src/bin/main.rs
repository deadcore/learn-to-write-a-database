use std::collections::HashMap;

use log::info;

use learn_to_write_a_database::backend::Backend;
use learn_to_write_a_database::backend::memory::InMemoryBackend;
use learn_to_write_a_database::statements::compiler::StatementCompiler;
use learn_to_write_a_database::statements::scanner::{Token, TokenIterator};
use learn_to_write_a_database::statements::Statement;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::builder().init();

    let mut backend = InMemoryBackend::new(
        HashMap::new()
    );

    let query_1 = r#"
    CREATE TABLE users (id INT, name TEXT);
    INSERT INTO users VALUES (1, 'Phil');
    INSERT INTO users VALUES (2, 'Kate');
    SELECT name FROM users;
    SELECT id FROM users;
    SELECT * FROM users;
    "#;

    let tokens = TokenIterator::new_iterator(
        query_1.chars()
    ).filter(|x| *x != Token::Space);

    let mut compiler = StatementCompiler::new(tokens);

    while let Some(maybe_statement) = compiler.next() {
        let statement = maybe_statement?;

        match statement {
            Statement::Create(statement) => {
                backend.create_table(&statement)?;
                info!("[create_table] ok")
            }
            Statement::Insert(statement) => {
                backend.insert(&statement)?;
                info!("[insert] ok")
            }
            Statement::Select(statement) => {
                let result = backend.select(&statement)?;
                info!("{:?}", result)
            }
        }
    }

    Ok(())
}

