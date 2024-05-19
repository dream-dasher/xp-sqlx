use sqlx::mysql::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect("mysql://root:root@127.0.0.1/university").await?;

    let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM students")
        .fetch_one(&pool)
        .await?;
    println!("Number of students: {}", row.0);

    let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM professors")
        .fetch_one(&pool)
        .await?;
    println!("Number of professors: {}", row.0);

    let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM courses")
        .fetch_one(&pool)
        .await?;
    println!("Number of courses: {}", row.0);

    let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM enrollments")
        .fetch_one(&pool)
        .await?;
    println!("Number of enrollments: {}", row.0);

    Ok(())
}
