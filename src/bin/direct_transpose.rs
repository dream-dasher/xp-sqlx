use chrono::NaiveDate;
use futures::TryStreamExt;
use sqlx::mysql::MySqlPoolOptions;

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
    println!("field StudentId:\n{:?}\n", student_ids);
    println!("field FirstName:\n{:?}\n", first_names);
    println!("field LastName:\n{:?}\n", last_names);
    println!("field DateOfBirth:\n{:?}\n", dates_of_birth);
    println!("field School:\n{:?}\n", schools);
    println!("field Email:\n{:?}\n", emails);

    Ok(())
}
