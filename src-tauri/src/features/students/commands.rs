use sqlx::SqlitePool;
use tauri::State;

use crate::features::students;

use super::model::Student;

#[tauri::command]
pub async fn create_student(
    pool: State<'_, SqlitePool>,
    first_name: String,
    last_name: String,
) -> Result<Student, String> {
    let result = sqlx::query(
        "
        INSERT INTO students (first_name, last_name)
        VALUES (?, ?)
        ",
    )
    .bind(&first_name)
    .bind(&last_name)
    .execute(pool.inner())
    .await
    .map_err(|error| error.to_string())?; // el simbolo ? significa que expanda el error si
                                          // es que hay uno

    let student = Student {
        id: result.last_insert_rowid(),
        first_name,
        last_name,
    };

    Ok(student)
}

#[tauri::command]
pub async fn get_students(pool: State<'_, SqlitePool>) -> Result<Vec<Student>, String> {
    // cada fila la convertira en struct
    // Student
    let students = sqlx::query_as::<_, Student>(
        "
        SELECT id, first_name, last_name
        FROM students
        ORDER BY first_name ASC
        ",
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|error| error.to_string())?;

    Ok(students)
}
