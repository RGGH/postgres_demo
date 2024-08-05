use std::error::Error;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // PostgreSQL connection
    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=5432 user=new_user dbname=new_db password=new_password",
        NoTls,
    )
    .await?;

    // Spawn a task to handle the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Query to get all records from the "users" table
    let rows = client.query("SELECT * FROM users", &[]).await?;

    // Print out the rows
    for row in rows {
        let id: i32 = row.get("id");
        let name: &str = row.get("name");
        println!("ID: {}, Name: {}", id, name);
    }

    Ok(())
}
