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

/// Student to use with `query!`
///
/// More work, and more potential for mistakes
/// but more control than with `query_as!`
#[derive(Debug, Display)]
#[display("StudentQ:{} Name: {} {} Born: {}",
          "id",
          "first_name",
          "last_name",
          "dob.map_or(\"N/A\".to_string(), |dob| dob.to_string())")]
struct StudentQA {
    // this is part of FromRow, which query_as! does not use
    // #[sqlx(rename = "StudentID")]
    id:         i32,
    first_name: String,
    last_name:  String,
    dob:        Option<NaiveDate>,
    school:     Option<String>,
    email:      Option<String>,
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
pub async fn v_of_struct_macro(repeat: u32) -> Result<(), sqlx::Error> {
    // Connection Pool
    let pool = MySqlPoolOptions::new().max_connections(5)
                                      .connect("mysql://root:root@127.0.0.1/university")
                                      .await?;

    let mut student_vec = Vec::new();
    for _ in 0..repeat {
        // let mut student_stream = sqlx::query_as!(StudentQA, "SELECT * FROM students").fetch(&pool);
        let mut student_stream = sqlx::query_as!(StudentQA,
                                                 r#"
                                                    SELECT StudentID as id, 
                                                           FirstName as first_name, 
                                                           LastName as last_name, 
                                                           DateOfBirth as dob, 
                                                           School as school, 
                                                           Email as email
                                                    FROM students 
                                                    "#,).fetch(&pool);
        while let Some(student) = student_stream.try_next().await? {
            student_vec.push(student);
        }
    }

    println!("{:?}", student_vec);

    #[allow(non_snake_case)]
    let df = struct_to_dataframe!(student_vec, [id, first_name, last_name, dob, school, email]);

    println!("\n\nDataframe:\n{:?}", df);

    Ok(())
}

/// Struct<vecs> ~~> Polars::DataFrame
#[inline]
pub async fn struct_of_v_macro(repeats: u32) -> Result<(), sqlx::Error> {
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
        // let mut student_stream = sqlx::query_as!(StudentQA, "SELECT * FROM students").fetch(&pool);
        let mut student_stream = sqlx::query_as!(StudentQA,
                                                 r#"
                                                    SELECT StudentID as id, 
                                                           FirstName as first_name, 
                                                           LastName as last_name, 
                                                           DateOfBirth as dob, 
                                                           School as school, 
                                                           Email as email
                                                    FROM students 
                                                    "#,).fetch(&pool);
        while let Some(student) = student_stream.try_next().await? {
            vstruct.student_id.push(student.id);
            vstruct.first_name.push(student.first_name);
            vstruct.last_name.push(student.last_name);
            vstruct.date_of_birth.push(student.dob);
            vstruct.school.push(student.school);
            vstruct.email.push(student.email);
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

pub async fn series_to_dataframe(repeats: u32) -> Result<(), sqlx::Error> {
    // Connection Pool
    let pool = MySqlPoolOptions::new().max_connections(5)
                                      .connect("mysql://root:root@127.0.0.1/university")
                                      .await?;

    let mut student_vec = Vec::new();
    for _ in 0..repeats {
        // let mut student_stream = sqlx::query_as!(StudentQA, "SELECT * FROM students").fetch(&pool);
        let mut student_stream = sqlx::query_as!(StudentQA,
                                                 r#"
                                                    SELECT StudentID as id, 
                                                           FirstName as first_name, 
                                                           LastName as last_name, 
                                                           DateOfBirth as dob, 
                                                           School as school, 
                                                           Email as email
                                                    FROM students 
                                                    "#,).fetch(&pool);
        while let Some(student) = student_stream.try_next().await? {
            student_vec.push(student);
        }
    }
    let ids = Series::new("id", student_vec.iter().map(|r| r.id).collect::<Vec<_>>());
    let first_names = Series::new("first_name",
                                  student_vec.iter()
                                             .map(|r| &r.first_name)
                                             .cloned()
                                             .collect::<Vec<_>>());
    let last_names = Series::new("last_name",
                                 student_vec.iter()
                                            .map(|r| &r.last_name)
                                            .cloned()
                                            .collect::<Vec<_>>());
    let dobs = Series::new("dob", student_vec.iter().map(|r| r.dob).collect::<Vec<_>>());
    let schools = Series::new("school",
                              student_vec.iter()
                                         .map(|r| r.school.clone())
                                         .collect::<Vec<_>>());
    let emails = Series::new("email",
                             student_vec.iter()
                                        .map(|r| r.email.clone())
                                        .collect::<Vec<_>>());

    // Create a DataFrame
    let df = DataFrame::new(vec![ids, first_names, last_names, dobs, schools, emails])
        .expect("f creation fine");
    println!("{:?}", df);

    Ok(())
}
