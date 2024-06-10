use chrono::NaiveDate;
use derive_more::Display;
use futures::TryStreamExt;
use polars::{frame::DataFrame, prelude::*};
use sqlx::{mysql::MySqlPoolOptions, Column, Execute, FromRow, Row};

/// Student to use with `query_as!`
///
/// FromRow is **not** used by query_as!
/// `query_as!` is rigid (and reliable) and easy
/// but lacks customization options
#[derive(Debug, Display)]
#[display(fmt = "StudentQA:{} Name: {} {} Born: {}",
          "StudentID",
          "FirstName",
          "LastName",
          "DateOfBirth.map_or(\"N/A\".to_string(), |dob| dob.to_string())")]
#[allow(non_snake_case)]
struct StudentQAMacro {
    // this is part of FromRow, which query_as! does not use
    // #[sqlx(rename = "StudentID")]
    StudentID:   i32,
    FirstName:   String,
    LastName:    String,
    DateOfBirth: Option<NaiveDate>,
    School:      Option<String>,
    Email:       Option<String>,
}
/// Student to use with `query!`
///
/// More work, and more potential for mistakes
/// but more control than with `query_as!`
#[derive(Debug, Display, FromRow)]
#[display(fmt = "StudentQ:{} Name: {} {} Born: {}",
          "id",
          "first_name",
          "last_name",
          "dob.map_or(\"N/A\".to_string(), |dob| dob.to_string())")]
struct StudentQAFunc {
    // this is part of FromRow, which query_as! does not use
    // #[sqlx(rename = "StudentID")]
    #[sqlx(rename = "StudentID")]
    id:         i32,
    #[sqlx(rename = "FirstName")]
    first_name: String,
    #[sqlx(rename = "LastName")]
    last_name:  String,
    #[sqlx(rename = "DateOfBirth")]
    dob:        Option<NaiveDate>,
    #[sqlx(rename = "School")]
    school:     Option<String>,
    #[sqlx(rename = "Email")]
    email:      Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Connection Pool
    let pool = MySqlPoolOptions::new().max_connections(5)
                                      .connect("mysql://root:root@127.0.0.1/university")
                                      .await?;

    let mut student_stream = sqlx::query_as!(StudentQAMacro, "SELECT * FROM students").fetch(&pool);
    while let Some(student) = student_stream.try_next().await? {
        println!("Student, {}", student);
    }

    // fetch + await-loop
    let mut rows = sqlx::query("SELECT * FROM professors").fetch(&pool);
    while let Some(row) = rows.try_next().await? {
        let first_name: &str = row.try_get("FirstName")?;
        println!("First name: {}", first_name);
    }

    let student_qa_macro = sqlx::query_as!(StudentQAMacro,
                                           "SELECT * FROM students WHERE StudentID =?",
                                           5).fetch_one(&pool)
                                             .await?;

    println!("-------------------------");
    println!("------ Query _ AS  ! (macro)------");
    println!("{}", student_qa_macro);
    println!("-------------------------");
    // ////////////////////////

    let student_qa_func: StudentQAFunc =
        sqlx::query_as("SELECT * FROM students WHERE StudentID = 5").fetch_one(&pool)
                                                                    .await?;

    println!("-------------------------");
    println!("------ Query _ AS (func)   ------");
    println!("{}", student_qa_func);
    println!("-------------------------");

    // Note: 'INT' minimally is i32, but may also be i64
    //       'ENUM' can be String
    let tuples: Vec<(i64, String, String, String)> = sqlx::query_as(
        "SELECT StudentID, FirstName, LastName, School FROM students WHERE StudentID >= ?",
    )
    .bind(62)
    .fetch_all(&pool)
    .await?;
    tuples.into_iter()
          .enumerate()
          .for_each(|(i, row)| println!("Student pull #{}: {:?}", i, row));

    // fetch_all + .get
    let rows = sqlx::query("SELECT * FROM enrollments WHERE EnrollmentID < ?").bind("5")
                                                                              .fetch_all(&pool)
                                                                              .await?;
    rows.into_iter().enumerate().for_each(|(i, row)| {
                                    println!("Enrollment pull #{}: {:?}",
                                             i,
                                             row.get::<String, &str>("Grade"))
                                });

    // fetch + await-loop
    let mut rows = sqlx::query("SELECT * FROM professors").fetch(&pool);
    while let Some(row) = rows.try_next().await? {
        let first_name: &str = row.try_get("FirstName")?;
        println!("First name: {}", first_name);
    }

    // WARN():
    // An "anon record" is returned with types that take the raw column names
    // so an `e.id` & `s.id` ~~> {id, id}
    // column names must be adjusted so as not to collide
    // (I believe this disambiguation will be needed even with the `query_as!`macro)
    let grade_floor = "A";
    let students_being_good = sqlx::query!(
                                           "
SELECT s.*, e.StudentID as eStudentID
FROM students s
JOIN enrollments e ON e.StudentID = s.StudentID
WHERE e.Grade = ?
        ",
                                           grade_floor
    ).fetch_all(&pool)
                              .await?;
    students_being_good.into_iter()
                       .enumerate()
                       .for_each(|(i, row)| println!("Student pull #{}:\n    {:?}", i, row));

    // Note: `Row` retains immutable borrow on `Conn` ∴ only 1 Row may ∃
    //       However, `row`, is merely a tuple of primitives here
    let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM students").fetch_one(&pool)
                                                                     .await?;
    println!("Number of students: {}", row.0);

    // ///////////////////////////////////////////////// //
    //playing with row response
    println!();
    let sql = sqlx::query_unchecked!("SELECT * FROM enrollments WHERE EnrollmentID = ?", 5).sql();
    println!("{}", sql);

    let row_m1 =
        sqlx::query!("SELECT * FROM enrollments WHERE EnrollmentID = ?", 1).fetch_one(&pool)
                                                                           .await?;
    println!("row_m1: {:?}", &row_m1);

    let row_1 = sqlx::query("SELECT * FROM enrollments WHERE EnrollmentID = ?").bind("5")
                                                                               .fetch_one(&pool)
                                                                               .await?;
    let row_2 = sqlx::query("SELECT * FROM enrollments WHERE EnrollmentID = ?").bind("6")
                                                                               .fetch_one(&pool)
                                                                               .await?;
    let cols: Vec<_> = row_1.columns()
                            .iter()
                            .map(|col| col.name().to_string())
                            .collect();

    println!("---------------");
    println!("{:?}", row_2);

    // query -> columns
    // for each column -> type, name
    // df with above
    Ok(())
}

#[allow(dead_code)]
/// TODO(match School ENUM in students)
enum School {
    Sciences,
    Humanities,
    Other,
}
