use std::collections::HashMap;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use crate::model::Class;


pub async fn init_db() -> Result<Pool<MySql>, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").expect("Database url must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    return Ok(pool);
}

pub async fn create_groups(classes_data: &HashMap<String, HashMap<String, Vec<Class>>>, pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    
}


