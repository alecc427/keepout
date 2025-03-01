use tokio::net::TcpSocket;
use std::net::SocketAddr;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use rpassword::read_password;
use argon2::{Argon2, password_hash::{rand_core::OsRng, PasswordHasher, SaltString}};

extern crate database;
use database::Database;

const PROMPT: &str = "<?> ";
const USERS_DB_URL: &str = "sqlite://./stayout.sqlite";

pub async fn start_comms() -> Result<SocketAddr, Box<dyn Error>>
{
    let local_host = "127.0.0.1:0".parse().unwrap();
    let chat_socket =TcpSocket::new_v4()?;

    // Set options for the chat socket
    chat_socket.set_keepalive(true)?;
    chat_socket.set_reuseaddr(true)?;
    chat_socket.set_reuseport(true)?;
    chat_socket.bind(local_host)?;

    let _ = get_username().await;
    let _to_send = get_user_msg().await;

    Ok(chat_socket.local_addr().unwrap())
}

async fn get_username() -> Result<(), Box<dyn Error>>
{
    print!("Enter your username: ");
    stdout().flush().unwrap();

    let mut username = String::new();
    stdin().read_line(&mut username).expect("Failed to read username");
    username = username.trim().to_string();

    let users_db = Database::new(&USERS_DB_URL).await.expect("Failed to open up the database file.");
    let user_exists = users_db.user_exists(&username).await.unwrap();

    if user_exists == true
    {
        check_password(&users_db, &username).await.unwrap();
    } else 
    {
        set_password(&users_db, &username).await.unwrap();
    }

    Ok(())
}

async fn check_password(users_db: &Database, username: &str) -> Result<(), Box<dyn Error>>
{
    print!("Enter your password: ");
    stdout().flush().unwrap();

    match read_password()
    {
        Ok(secret) => 
        {
            let pwd_hash = hash_password(&secret.trim().to_string()).await.unwrap();
            let db_pwd = users_db.get_user_password(&pwd_hash).await.unwrap();

            if db_pwd == pwd_hash
            {
                Ok(())
            } else 
            {
                println!("Password incorrect. Try again");
                return Box::pin(check_password(users_db, username)).await;
            }
        },

        Err(_) => todo!(),
    }
}

async fn set_password(users_db: &Database, username: &str) -> Result<(), Box<dyn Error>>
{
    print!("Enter a password for your new account: ");
    stdout().flush().unwrap();

    match read_password()
    {
        Ok(secret) => 
        {
            let pwd_hash = hash_password(&secret.trim().to_string()).await.unwrap();
            let _ = users_db.insert_user(username, &pwd_hash).await;
            Ok(())
        },

        Err(_) => todo!(),
    }
}

pub async fn get_user_msg() -> Result<String, Box<dyn Error>>
{
    print!("{}", PROMPT);
    stdout().flush().unwrap();

    let mut msg = String::new();
    stdin().read_line(&mut msg).expect("Failed to read input");

    Ok(msg)
}

async fn hash_password(password: &str) -> Result<String, Box<dyn Error>> 
{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    Ok(password_hash)
}
