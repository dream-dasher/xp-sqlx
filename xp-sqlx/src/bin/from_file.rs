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
/// Student to use with `query!`
///
/// More work, and more potential for mistakes
/// but more control than with `query_as!`
#[derive(Debug, Display)]
#[display(fmt = "StudentQ:{} Name: {} {} Born: {}",
          "id",
          "first_name.clone().unwrap_or_default()",
          "last_name.clone().unwrap_or_default()",
          "dob.map_or(\"N/A\".to_string(), |dob| dob.to_string())")]
struct StudentQA {
    // this is part of FromRow, which query_as! does not use
    // #[sqlx(rename = "StudentID")]
    id:         i32,
    first_name: Option<String>,
    last_name:  Option<String>,
    dob:        Option<NaiveDate>,
    school:     Option<String>,
    email:      Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new().max_connections(2)
                                      .connect("mysql://root:root@127.0.0.1/university")
                                      .await?;
    println!("~~~~~~~~~~~~~~~");
    students_10_qa(&pool).await?;

    println!("~~~~~~~~~~~~~~~");
    students_wid_qa(&pool).await?;

    println!("~~~~~~~~~~~~~~~");
    students_10_q(&pool).await?;

    Ok(())
}

// Options : A, B, C
// get selection "x"
// if x --> fetch.file(x), fetch.pre-query(x), fetch.params(x)
// present file, pre-query, params
// get params "q,r,c"
// if "x,y,c" --> query --> polars::DF

/// query_as!
/// WARN: file paths must be given literally
///       there are no ergonomic management options
async fn students_10_qa(pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    let mut stream =
        sqlx::query_file_as!(StudentQA, "../data/sql_queries/students_10.sql").fetch(pool);
    while let Some(student) = stream.try_next().await? {
        println!("Student, {}", student);
    }
    Ok(())
}

/// query_as!
async fn students_wid_qa(pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    let mut stream =
        sqlx::query_file_as!(StudentQA, "../data/sql_queries/students_w_id.sql", 12).fetch(pool);
    while let Some(student) = stream.try_next().await? {
        println!("Student, {}", student);
    }
    Ok(())
}

/// query!
async fn students_10_q(pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    let mut stream = sqlx::query_file!("../data/sql_queries/students_10.sql").fetch(pool);
    while let Some(row) = stream.try_next().await? {
        println!("Student, {:?}", row);
    }
    Ok(())
}
