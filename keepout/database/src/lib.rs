use sqlx::{Pool, Sqlite, SqlitePool, Error};
pub struct Database 
{
    pool: Pool<Sqlite>,
}

impl Database {
    // Initialize and connect to the database
    pub async fn new(db_url: &str) -> Result<Self, Error> 
    {
        let pool = SqlitePool::connect(db_url).await?;
        Ok(Self { pool })
    }

    pub async fn user_exists(&self, username: &str) -> Result<bool, Error> 
    {
        let result: (u8,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(result.0 > 0)
    }

    pub async fn insert_user(&self, username: &str, password: &str) -> Result<(), Error> 
    {
        sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
            .bind(username)
            .bind(password)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_user_password(&self, username: &str) -> Result<String, Error> 
    {
        let result_pwd:(String,)= sqlx::query_as("SELECT password FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;

        Ok(result_pwd.0)
    }
}
