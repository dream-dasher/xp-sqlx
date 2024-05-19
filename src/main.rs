use sqlx::mysql::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect("mysql://root:root@127.0.0.1/student").await?;

    let row: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM students")
        .fetch_one(&pool)
        .await?;

    println!("Number of students: {}", row.0);

    Ok(())
}
