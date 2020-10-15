use std::iter::Peekable;

use log::{debug, trace};

use crate::statements::{insert, Statement, select};
use crate::statements::create::{ColumnDefinition, CreateTableStatement, DataType};
use crate::statements::insert::InsertStatement;
use crate::statements::scanner::{KeywordToken, Token};
use crate::statements::select::SelectStatement;

pub struct StatementCompiler<T: Iterator<Item=Token>> {
    inner: Peekable<T>,
}

impl<T: Iterator<Item=Token>> StatementCompiler<T> {
    pub fn new(tokens: T) -> Self {
        StatementCompiler {
            inner: tokens.peekable()
        }
    }

    fn compile_create_table(&mut self) -> crate::Result<Statement> {
        let identifier = self.assert_next_identifier()?;

        return Ok(
            Statement::Create(
                CreateTableStatement::new(
                    identifier,
                    self.compile_create_table_column_definitions()?,
                )
            )
        );
    }

    fn compile_create_table_column_definitions(&mut self) -> crate::Result<Vec<ColumnDefinition>> {
        self.repeat_statement(|stream| {
            let name = stream.assert_next_identifier()?;
            let data_type = stream.read_data_type()?;

            Ok(ColumnDefinition::new(
                name,
                data_type,
            ))
        })
    }

    fn compile_create_table_column_definition(&mut self) -> crate::Result<ColumnDefinition> {
        let name = self.assert_next_identifier()?;
        let data_type = self.read_data_type()?;

        Ok(ColumnDefinition::new(
            name,
            data_type,
        ))
    }

    fn read_data_type(&mut self) -> crate::Result<DataType> {
        match self.inner.next() {
            Some(Token::Keyword(KeywordToken::INT)) => Ok(DataType::Int32),
            Some(Token::Keyword(KeywordToken::TEXT)) => Ok(DataType::String),
            Some(token) => Err(format!("Expected a datatype but got {:?}", token).into()),
            None => Err("Expected an identifier but got nothing".into()),
        }
    }

    fn compile_insert(&mut self) -> crate::Result<Statement> {
        self.assert_next_token_is(Token::Keyword(KeywordToken::INTO))?;

        let identifier = self.assert_next_identifier()?;

        self.assert_next_token_is(Token::Keyword(KeywordToken::VALUES))?;

        let expressions = self.repeat_statement(|stream| {
            match stream.inner.next() {
                Some(Token::U32(value)) => {
                    Ok(insert::Expression::Literal(insert::Literal::U32(value)))
                }
                Some(Token::Apostrophe) => {
                    let string_literal = stream.assert_next_identifier()?;
                    stream.assert_next_token_is(Token::Apostrophe)?;

                    Ok(insert::Expression::Literal(insert::Literal::String(string_literal)))
                }
                Some(unhandled) => Err(format!("Unhandled token: {:?}", unhandled).into()),
                None => Err("Expected a literal value but got nothing".into())
            }
        })?;

        Ok(Statement::Insert(InsertStatement::new(identifier, expressions)))
    }

    fn repeat_statement<F, R>(&mut self, mut f: F) -> crate::Result<Vec<R>>
        where
            F: FnMut(&mut Self) -> crate::Result<R>,
    {
        self.assert_next_token_is(Token::LeftBracket)?;

        let results = self.repeat_vargs_statement(f)?;

        self.assert_next_token_is(Token::RightBracket)?;

        Ok(results)
    }

    fn repeat_vargs_statement<F, R>(&mut self, mut f: F) -> crate::Result<Vec<R>>
        where
            F: FnMut(&mut Self) -> crate::Result<R>,
    {
        let mut results = Vec::new();

        results.push(f(self)?);

        while let Some(Token::Comma) = self.inner.peek() {
            self.skip();
            results.push(f(self)?);
        }

        Ok(results)
    }

    fn compile_create(&mut self) -> crate::Result<Statement> {
        trace!("Compiling create statement");

        self.assert_next_token_is(Token::Keyword(KeywordToken::TABLE))?;
        self.compile_create_table()
    }

    fn compile_select(&mut self) -> crate::Result<Statement> {
        let identifiers = self.repeat_vargs_statement(|stream| {
            match stream.inner.next() {
                Some(Token::Identifier(identifier)) => Ok(select::Expression::Column(identifier)),
                Some(Token::Asterisk) => Ok(select::Expression::All),
                Some(unhandled) => Err(format!("Unhandled token: {:?}", unhandled).into()),
                None => Err("Expected a literal value but got nothing".into())
            }
        })?;

        self.assert_next_token_is(Token::Keyword(KeywordToken::FROM))?;

        let table = self.assert_next_identifier()?;

        Ok(Statement::Select(SelectStatement::new(identifiers, table)))
    }

    fn assert_next_token_is(&mut self, item: Token) -> crate::Result<Token> {
        if let Some(v) = self.inner.next() {
            return if v == item {
                Ok(v)
            } else {
                Err(format!("Expected a {:?} but got {:?}", item, v).into())
            };
        }
        return Err("Expected a token but got nothing".into());
    }

    fn assert_next_identifier(&mut self) -> crate::Result<String> {
        match self.inner.next() {
            Some(Token::Identifier(identifier)) => Ok(identifier),
            Some(token) => Err(format!("Expected an identifier but got {:?}", token).into()),
            None => Err("Expected an identifier but got nothing".into())
        }
    }

    fn skip(&mut self) {
        let token = self.inner.next();
        trace!("Skipping token: {:?}", token)
    }
}

impl<T: Iterator<Item=Token>> Iterator for StatementCompiler<T> {
    type Item = crate::Result<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(token) = self.inner.next() {
            trace!("popped a token: {:?}", token);

            match token {
                Token::Keyword(KeywordToken::CREATE) => return Some(self.compile_create()),
                Token::Keyword(KeywordToken::INSERT) => return Some(self.compile_insert()),
                Token::Keyword(KeywordToken::SELECT) => return Some(self.compile_select()),
                Token::SemiColon => {} //skip
                Token::NewLine => {} //skip
                unhandled => return Some(Err(format!("Unable to compile keyword: [{:?}]. It looks the compiler does not understand it", unhandled).into())),
            };
        }
        return None;
    }
}