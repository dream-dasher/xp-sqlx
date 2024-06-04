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

#[derive(Debug)]
struct VecOfStudentQA {
    pub student_id: Vec<Option<i32>>,
    pub first_name: Vec<Option<String>>,
    pub last_name: Vec<Option<String>>,
    pub date_of_birth: Vec<Option<NaiveDate>>,
    pub school: Vec<Option<String>>,
    pub email: Vec<Option<String>>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Connection Pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@127.0.0.1/university")
        .await?;

    // making vecs manually
    let mut student_ids: Vec<Option<i32>> = Vec::new();
    let mut first_names: Vec<Option<String>> = Vec::new();
    let mut last_names: Vec<Option<String>> = Vec::new();
    let mut dates_of_birth: Vec<Option<NaiveDate>> = Vec::new();
    let mut schools: Vec<Option<String>> = Vec::new();
    let mut emails: Vec<Option<String>> = Vec::new();

    let mut student_stream = sqlx::query_as!(StudentQA, "SELECT * FROM students").fetch(&pool);
    while let Some(student) = student_stream.try_next().await? {
        student_ids.push(student.StudentID);
        first_names.push(student.FirstName);
        last_names.push(student.LastName);
        dates_of_birth.push(student.DateOfBirth);
        schools.push(student.School);
        emails.push(student.Email);
    }
    dbg!(student_ids);
    dbg!(first_names);
    dbg!(last_names);
    dbg!(dates_of_birth);
    dbg!(schools);
    dbg!(emails);

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

    Ok(())
}
