use chrono::NaiveDate;
use derive_more::Display;
use divan;
use futures::TryStreamExt;
use polars::prelude::*;
use sqlx::mysql::MySqlPoolOptions;

/// Takes a Vec<Struct> and generates a Polars::DataFrame
/// by way of multiple Vec<field_x>s
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
#[derive(Debug)]
#[allow(non_snake_case)]
struct StudentQA {
    StudentID: Option<i32>,
    FirstName: Option<String>,
    LastName: Option<String>,
    DateOfBirth: Option<NaiveDate>,
    School: Option<String>,
    Email: Option<String>,
}

fn main() {
    divan::main();
}

#[divan::bench()]
async fn recopy_transpose() -> Result<(), sqlx::Error> {
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

// ////////////////////////////// //

/// Struct of Vecs into a Polars::DataFrame
/// Simply passing the fields into the `df!` macro with labels
macro_rules! vstruct_to_dataframe {
    ($input:expr, [$($field:ident),+]) => {
        {
            df! {
                $(stringify!($field) => $input.$field,)*
            }
        }
    };
}

/// Vecs of each field
/// to transpose the memory representation
#[derive(Debug)]
struct VecOfStudentQA {
    pub student_id: Vec<Option<i32>>,
    pub first_name: Vec<Option<String>>,
    pub last_name: Vec<Option<String>>,
    pub date_of_birth: Vec<Option<NaiveDate>>,
    pub school: Vec<Option<String>>,
    pub email: Vec<Option<String>>,
}

#[divan::bench()]
async fn vstruct_transpose() -> Result<(), sqlx::Error> {
    // Connection Pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@127.0.0.1/university")
        .await?;

    let mut vstruct = VecOfStudentQA {
        student_id: Vec::new(),
        first_name: Vec::new(),
        last_name: Vec::new(),
        date_of_birth: Vec::new(),
        school: Vec::new(),
        email: Vec::new(),
    };
    let mut student_stream = sqlx::query_as!(StudentQA, "SELECT * FROM students").fetch(&pool);
    while let Some(student) = student_stream.try_next().await? {
        vstruct.student_id.push(student.StudentID);
        vstruct.first_name.push(student.FirstName);
        vstruct.last_name.push(student.LastName);
        vstruct.date_of_birth.push(student.DateOfBirth);
        vstruct.school.push(student.School);
        vstruct.email.push(student.Email);
    }

    println!("{:?}", vstruct);

    let df = vstruct_to_dataframe!(
        vstruct,
        [
            student_id,
            first_name,
            last_name,
            date_of_birth,
            school,
            email
        ]
    );

    println!("\n\nDataframe:\n{:?}", df);

    Ok(())
}
