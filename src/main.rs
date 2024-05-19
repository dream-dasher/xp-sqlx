use futures::TryStreamExt;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Connection Pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@127.0.0.1/university")
        .await?;

    // Note: 'INT' minimally is i32, but may also be i64
    //       'ENUM' can be String
    let tuples: Vec<(i64, String, String, String)> = sqlx::query_as(
        "SELECT StudentID, FirstName, LastName, School FROM students WHERE StudentID >= ?",
    )
    .bind(2)
    .fetch_all(&pool)
    .await?;
    tuples
        .into_iter()
        .enumerate()
        .for_each(|(i, row)| println!("Student pull #{}: {:?}", i, row));

    // fetch_all + .get
    let rows = sqlx::query("SELECT * FROM enrollments WHERE EnrollmentID < ?")
        .bind("5")
        .fetch_all(&pool)
        .await?;
    rows.into_iter().enumerate().for_each(|(i, row)| {
        println!(
            "Enrollment pull #{}: {:?}",
            i,
            row.get::<String, &str>("Grade")
        )
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
    )
    .fetch_all(&pool)
    .await?;
    students_being_good
        .into_iter()
        .enumerate()
        .for_each(|(i, row)| println!("Student pull #{}:\n    {:?}", i, row));

    // Note: `Row` retains immutable borrow on `Conn` ∴ only 1 Row may ∃
    //       However, `row`, is merely a tuple of primitives here
    let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM students")
        .fetch_one(&pool)
        .await?;
    println!("Number of students: {}", row.0);

    Ok(())
}

/// TODO(match School ENUM in students)
enum School {
    Sciences,
    Humanities,
    Other,
}
