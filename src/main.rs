use anyhow::Result;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::FromRow)]
struct Student<T> {
    student_name: T,
    class: i32,
}

impl<T> Student<T> {
    fn new(name: T, class: i32) -> Self {
        Student {
            student_name: name,
            class,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    sqlx::query!("CREATE TABLE IF NOT EXISTS students (student_id serial PRIMARY KEY, student_name VARCHAR (50) NOT NULL, class INT NOT NULL)")
        .execute(&pool).await?;

    let student = Student::new("Jack", 1);

    let row: (i32,) = sqlx::query_as(
        "INSERT INTO students (student_name, class) values ($1, $2) returning student_id",
    )
    .bind(student.student_name)
    .bind(student.class)
    .fetch_one(&pool)
    .await?;
    println!("{:?}", row);
    Ok(())
}
