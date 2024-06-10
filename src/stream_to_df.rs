//! Various ways of getting an async SQLx stream into a polars::DataFrame

use chrono::NaiveDate;
use derive_more::Display;
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

/// Student to use with `query_as!`
///
/// FromRow is **not** used by query_as!
/// `query_as!` is rigid (and reliable) and easy
/// but lacks customization options
#[derive(Debug, Display)]
#[display(fmt = "StudentQA:{} Name: {} {} Born: {}",
          "StudentID",
          "FirstName.clone()",
          "LastName.clone()",
          "DateOfBirth.map_or(\"N/A\".to_string(), |dob| dob.to_string())")]
#[allow(non_snake_case)]
pub struct StudentQA {
    StudentID:   i32,
    FirstName:   String,
    LastName:    String,
    DateOfBirth: Option<NaiveDate>,
    School:      Option<String>,
    Email:       Option<String>,
}

/// Vecs of each field
/// to transpose the memory representation
#[derive(Debug)]
pub struct VecOfStudentQA {
    pub student_id:    Vec<i32>,
    pub first_name:    Vec<String>,
    pub last_name:     Vec<String>,
    pub date_of_birth: Vec<Option<NaiveDate>>,
    pub school:        Vec<Option<String>>,
    pub email:         Vec<Option<String>>,
}

/// Vec<Struct> ~~> Polars::DataFrame
#[inline]
pub async fn recopy_transpose(repeat: u32) -> Result<(), sqlx::Error> {
    // Connection Pool
    let pool = MySqlPoolOptions::new().max_connections(5)
                                      .connect("mysql://root:root@127.0.0.1/university")
                                      .await?;

    let mut student_vec = Vec::new();
    for _ in 0..repeat {
        let mut student_stream = sqlx::query_as!(StudentQA, "SELECT * FROM students").fetch(&pool);
        while let Some(student) = student_stream.try_next().await? {
            student_vec.push(student);
        }
    }

    println!("{:?}", student_vec);

    #[allow(non_snake_case)]
    let df = struct_to_dataframe!(student_vec, [StudentID,
                                                FirstName,
                                                LastName,
                                                DateOfBirth,
                                                School,
                                                Email]);

    println!("\n\nDataframe:\n{:?}", df);

    Ok(())
}

/// Struct<vecs> ~~> Polars::DataFrame
#[inline]
pub async fn vstruct_transpose(repeats: u32) -> Result<(), sqlx::Error> {
    // Connection Pool
    let pool = MySqlPoolOptions::new().max_connections(5)
                                      .connect("mysql://root:root@127.0.0.1/university")
                                      .await?;

    // I need a `new()` for this, lol
    // is there an easier way to get defaults...?
    let mut vstruct = VecOfStudentQA { student_id:    Vec::new(),
                                       first_name:    Vec::new(),
                                       last_name:     Vec::new(),
                                       date_of_birth: Vec::new(),
                                       school:        Vec::new(),
                                       email:         Vec::new(), };
    for _ in 0..repeats {
        let mut student_stream = sqlx::query_as!(StudentQA, "SELECT * FROM students").fetch(&pool);
        while let Some(student) = student_stream.try_next().await? {
            vstruct.student_id.push(student.StudentID);
            vstruct.first_name.push(student.FirstName);
            vstruct.last_name.push(student.LastName);
            vstruct.date_of_birth.push(student.DateOfBirth);
            vstruct.school.push(student.School);
            vstruct.email.push(student.Email);
        }
    }

    println!("{:?}", vstruct);

    let df = vstruct_to_dataframe!(vstruct, [student_id,
                                             first_name,
                                             last_name,
                                             date_of_birth,
                                             school,
                                             email]);

    println!("\n\nDataframe:\n{:?}", df);

    Ok(())
}

/// Directly creating n Vec<field>s then creating a DataFrame
#[inline]
pub async fn direct_transpose(repeat: u32) -> Result<(), sqlx::Error> {
    // Connection Pool
    let pool = MySqlPoolOptions::new().max_connections(5)
                                      .connect("mysql://root:root@127.0.0.1/university")
                                      .await?;

    // making vecs manually
    let mut student_ids: Vec<i32> = Vec::new();
    let mut first_names: Vec<String> = Vec::new();
    let mut last_names: Vec<String> = Vec::new();
    let mut dates_of_birth: Vec<Option<NaiveDate>> = Vec::new();
    let mut schools: Vec<Option<String>> = Vec::new();
    let mut emails: Vec<Option<String>> = Vec::new();

    for _ in 0..repeat {
        let mut student_stream = sqlx::query_as!(StudentQA, "SELECT * FROM students").fetch(&pool);
        while let Some(student) = student_stream.try_next().await? {
            student_ids.push(student.StudentID);
            first_names.push(student.FirstName);
            last_names.push(student.LastName);
            dates_of_birth.push(student.DateOfBirth);
            schools.push(student.School);
            emails.push(student.Email);
        }
    }
    println!("field StudentId:\n{:?}\n", student_ids);
    println!("field FirstName:\n{:?}\n", first_names);
    println!("field LastName:\n{:?}\n", last_names);
    println!("field DateOfBirth:\n{:?}\n", dates_of_birth);
    println!("field School:\n{:?}\n", schools);
    println!("field Email:\n{:?}\n", emails);

    Ok(())
}
