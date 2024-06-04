use chrono::NaiveDate;
use derive_more::Display;
use futures::{StreamExt, TryStreamExt};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{FromRow, Row};

/// Student to use with `query_as!`
///
/// FromRow is **not** used by query_as!
/// `query_as!` is rigid (and reliable) and easy
/// but lacks customization options
#[derive(Debug, Display)]
#[display(
    fmt = "StudentQA:{} Name: {} {} Born: {}",
    "StudentID.unwrap_or_default()",
    "FirstName.clone().unwrap_or_default()",
    "LastName.clone().unwrap_or_default()",
    "DateOfBirth.map_or(\"N/A\".to_string(), |dob| dob.to_string())"
)]
#[allow(non_snake_case)]
struct StudentQA {
    StudentID: Option<i32>,
    FirstName: Option<String>,
    LastName: Option<String>,
    DateOfBirth: Option<NaiveDate>,
    School: Option<String>,
    Email: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Connection Pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@127.0.0.1/university")
        .await?;

    let mut student_vec = Vec::new();
    let mut student_stream = sqlx::query_as!(StudentQA, "SELECT * FROM students").fetch(&pool);
    while let Some(student) = student_stream.try_next().await? {
        student_vec.push(student);
    }

    println!("{:?}", student_vec);

    Ok(())
}
