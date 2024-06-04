use chrono::NaiveDate;
use derive_more::Display;
use futures::TryStreamExt;
use polars::prelude::*;
use sqlx::mysql::MySqlPoolOptions;

macro_rules! struct_to_dataframe {
    ($input:expr, [$($field:ident),+]) => {
        {
            let len = $input.len().to_owned();

            // Extract the field values into separate vectors
            $(let mut $field = Vec::with_capacity(len);)*

            for e in $input.into_iter() {
                $($field.push(e.$field);)*
            }
            df! {
                $(stringify!($field) => $field,)*
            }
        }
    };
}

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

    #[allow(non_snake_case)]
    let df = struct_to_dataframe!(
        student_vec,
        [StudentID, FirstName, LastName, DateOfBirth, School, Email]
    );

    println!("\n\nDataframe:\n{:?}", df);

    Ok(())
}
