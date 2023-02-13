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

struct InsertStudent {
    student_name: String,
    class: i32,
}

async fn insert_student(pool: &PgPool, student: InsertStudent) -> Result<i32> {
    let entry = sqlx::query!(
        "INSERT INTO students (student_name, class) VALUES ($1, $2) RETURNING student_id",
        student.student_name,
        student.class,
    )
    .fetch_one(pool)
    .await?;
    Ok(entry.student_id)
}

async fn get_student(pool: &PgPool, id: i32) -> Result<Student> {
    let student = sqlx::query_as!(Student, "SELECT * FROM students WHERE student_id = $1", id)
        .fetch_one(pool)
        .await?;
    Ok(student)
}

async fn delete_student(pool: &PgPool, id: i32) -> Result<()> {
    sqlx::query!("DELETE FROM students where student_id = $1", id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn update_student(pool: &PgPool, id: i32, student_name: String, class: i32) -> Result<()> {
    sqlx::query!(
        "UPDATE students set student_name=$1, class=$2 where student_id=$3",
        student_name,
        class,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
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
    let new_student = InsertStudent {
        student_name: format!("Johnny"),
        class: 11,
    };
    if let Ok(st_id) = insert_student(&pool, new_student).await {
        println!("Insertion successful! Id: {}", st_id);
    }
    if let Ok(student_details) = get_student(&pool, 3).await {
        println!("{}", serde_json::to_string(&student_details)?);
    }
    if let Ok(_) = delete_student(&pool, 5).await {
        println!("Deletion successful!");
    }
    if let Ok(_) = update_student(&pool, 10, format!("Harry"), 3).await {
        println!("Update successful!");
    }
    Ok(())
}
