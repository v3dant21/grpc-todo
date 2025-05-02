use serde::{Serialize, Deserialize};
use sqlx::{migrate::MigrateDatabase, Pool,Sqlite,SqlitePool};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize ,sqlx::FromRow)]
pub struct Todo{
    pub id:Option<i64>,
    pub title:Option<String>,
    pub completed:bool,

}

pub async fn init_db()-> Pool<Sqlite> {
    let config = Config::init();
    if !Sqlite::database_exists(&config.database_url).await.unwrap_or(false) {
        println!("creating database{}",&config.database_url);
        match Sqlite::create_database(&config.database_url).await{
            Ok(_)=> println!("created db sucess"),
            Err(error) =>println!("error:{}",error),

        }
    }else{
        println!("db already exists");
    }
    let pool = SqlitePool::connect( &config.database_url).await.unwrap();
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let migrations =std::path::Path::new(&crate_dir).join("migrations");

    let migration_results =sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&pool)
        .await;
    match migration_results{
        Ok(_)=> println!("Migration sucess"),
        Err(error)=>println!("ERROR{}", error),
    }

    pool
}

pub async fn create_todo(pool:&Pool<Sqlite>, title: &str)-> Todo{
    sqlx::query_as::<_,Todo>(
            "INSERT INTO todo (title, completed) VALUES (?,?) RETURNING id, title, completed"
    )
        .bind(title)
        .bind(false)
        .fetch_one(pool)
        .await
        .expect("failed to insert todo")
}

pub async fn get_todos(pool:&Pool<Sqlite>)->Vec<Todo>{
    sqlx::query_as::<_, Todo>("SELECT id , title, completed FROM Todo")
    .fetch_all(pool)
    .await
    .expect("failed to fetch todo")

}
pub async fn update_todo(pool:&Pool<Sqlite>, id:Option<i64>, title:&str, completed:bool)-> Todo{
    sqlx::query_as::<_,Todo>(
        "UPDATE todo SET title = ?, completed = ? WHERE id = ? RETURNING id, title, completed"
    )
    .bind(title)
    .bind(completed)
    .bind(id)
    .fetch_one(pool)
    .await
    .expect("failed to update todo")
}

pub async fn delete_todo(pool: &Pool<Sqlite>, id: Option<i64>) {
    sqlx::query("DELETE FROM todo WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .expect("Failed to delete todo");
}





