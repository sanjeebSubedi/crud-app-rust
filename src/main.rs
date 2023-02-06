use anyhow::Result;
use dotenv::dotenv;
use serde::Serialize;
use serde_json;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(sqlx::FromRow, Debug, Serialize)]
struct Student {
    student_id: i32,
    student_name: String,
    class: i32,
}

impl Student {
    fn new(student_id: i32, name: String, class: i32) -> Self {
        Student {
            student_id,
            student_name: name,
            class,
        }
    }
}

async fn insert_student(pool: &PgPool, student_name: &str, class: i32) -> Result<i32> {
    let entry: (i32,) = sqlx::query_as(
        "INSERT INTO students (student_name, class) VALUES ($1, $2) RETURNING student_id",
    )
    .bind(student_name)
    .bind(class)
    .fetch_one(pool)
    .await?;
    Ok(entry.0)
}

async fn get_student(pool: &PgPool, id: i32) -> Result<Student> {
    let student = sqlx::query_as("SELECT * FROM students WHERE student_id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(student)
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
    if let Ok(st_id) = insert_student(&pool, "Jackson", 1).await {
        println!("Insertion successful! Id: {}", st_id);
    }
    if let Ok(student_details) = get_student(&pool, 3).await {
        println!("{}", serde_json::to_string(&student_details)?);
    }
    Ok(())
}
