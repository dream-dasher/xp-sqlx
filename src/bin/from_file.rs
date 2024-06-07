//! Using queries from a file.
//! Parameterized and non-parameterized.
//!
//! # Note:
//! - the `query*!` macros require literal string tokens for the path
//!     - `const` &str will *not* suffice (const are defined after macro expansion)
//! - this sucks, but it's just what we have to work with unless we want to define our own procedural macros
//! - We *could* define functions to wrap the queries with raw strings.

use chrono::NaiveDate;
use derive_more::Display;
use futures::TryStreamExt;
use sqlx::mysql::MySqlPoolOptions;

/// Student to use with `query_as!`
///
/// FromRow is **not** used by query_as!
/// `query_as!` is rigid (and reliable) and easy
/// but lacks customization options
#[derive(Debug, Display, sqlx::FromRow)]
#[display(fmt = "StudentQA:{} Name: {} {} Born: {}",
          "StudentID.unwrap_or_default()",
          "FirstName.clone().unwrap_or_default()",
          "LastName.clone().unwrap_or_default()",
          "DateOfBirth.map_or(\"N/A\".to_string(), |dob| dob.to_string())")]
#[allow(non_snake_case)]
struct StudentQA {
    // this is part of FromRow, which query_as! does not use
    // #[sqlx(rename = "StudentID")]
    StudentID:   Option<i32>,
    FirstName:   Option<String>,
    LastName:    Option<String>,
    DateOfBirth: Option<NaiveDate>,
    School:      Option<String>,
    Email:       Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new().max_connections(2)
                                      .connect("mysql://root:root@127.0.0.1/university")
                                      .await?;

    // query_as!

    println!("-------------------");
    // WARN: file paths must be given literally
    //       there are no ergonomic management options
    let mut stream =
        sqlx::query_file_as!(StudentQA, "data/sql_queries/students_10.sql").fetch(&pool);
    while let Some(student) = stream.try_next().await? {
        print!("Student, {}", student);
    }

    println!("\n-------------------");
    let mut stream =
        sqlx::query_file_as!(StudentQA, "data/sql_queries/students_w_id.sql", 12).fetch(&pool);
    while let Some(student) = stream.try_next().await? {
        println!("Student, {}", student);
    }

    // query!
    println!("-------------------");
    println!("-------------------");
    let mut stream = sqlx::query_file!("data/sql_queries/students_10.sql").fetch(&pool);
    while let Some(row) = stream.try_next().await? {
        println!("Student, {:?}", row);
    }

    Ok(())
}
