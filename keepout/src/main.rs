use tokio::signal;
use tokio::task;

extern crate client;

#[tokio::main]
async fn main()
{
    let task_handler = task::spawn(async 
        {
            let chat_socket = client::start_comms().await.unwrap();

            loop 
            {
                let _ = client::get_user_msg().await;
            }
        });

    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("\nShutdown signal received. Closing chat. Goodbye :)");
    task_handler.abort();

    std::process::exit(0);
}
