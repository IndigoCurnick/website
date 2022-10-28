use once_cell::sync::OnceCell;
use std::env;
use std::sync::Arc;
use tokio_postgres::Client;
use tokio_postgres::NoTls;
pub static POSTGRES_CLIENT: OnceCell<Arc<Client>> = OnceCell::new();

pub async fn get_inited_pg_client() -> Result<&'static Arc<Client>, String> {
    match pg_init().await {
        Ok(()) => {}
        Err(()) => return Err("Could not initialise Postgress client".to_string()),
    };

    let pg_client = match POSTGRES_CLIENT.get() {
        Some(x) => x,
        None => return Err("Postgres was previously inited but no longer available".to_string()),
    };

    return Ok(pg_client);
}

pub async fn insert_to_database(domain: String, webpage: String) {
    if cfg!(debug_assertions) {
        return;
    }
    let client = match get_inited_pg_client().await {
        Ok(x) => x,
        Err(y) => panic!("{}", y),
    };

    let insert_text = format!(
        "INSERT INTO visits (domain, webpage) VALUES ('{}', '{}')",
        domain, webpage
    );

    match client.execute(&insert_text, &[]).await {
        Ok(x) => println!("Has gone into database"),
        Err(y) => println!("{}", y),
    };
}

pub async fn pg_init() -> Result<(), ()> {
    if POSTGRES_CLIENT.get().is_none() {
        let pg_client = get_pg_client(
            "PG_URI".to_string(),
            "PG_USERNAME".to_string(),
            "PG_PASSWORD".to_string(),
            "PG_DB".to_string(),
        )
        .await;

        if pg_client.is_err() {
            panic!("Error getting pg client: {:?}", pg_client);
        }

        let set_res = POSTGRES_CLIENT.set(Arc::new(pg_client.unwrap()));

        if set_res.is_err() {
            let message = format!("Could not connect to pg while validating: {:?}", set_res);
            error!("{}", message);
            return Err(());
        }

        return Ok(());
    } else {
        return Ok(());
    }
}

async fn get_pg_client(
    pg_uri_env: String,
    pg_username_env: String,
    pg_password_env: String,
    pg_db_env: String,
) -> Result<Client, String> {
    let pg_uri;
    match env::var_os(pg_uri_env.clone()) {
        Some(x) => pg_uri = x,
        None => {
            let msg = format!("No {} environment variable set!", pg_uri_env);
            error!("{}", msg);
            return Err(msg.to_string());
        }
    };

    let pg_username;
    match env::var_os(pg_username_env.clone()) {
        Some(x) => pg_username = x,
        None => {
            let msg = format!("No {} environment variable set!", pg_username_env);
            error!("{}", msg);
            return Err(msg.to_string());
        }
    };

    let pg_password;
    match env::var_os(pg_password_env.clone()) {
        Some(x) => pg_password = x,
        None => {
            let msg = format!("No {} environment variable set!", pg_password_env);
            error!("{}", msg);
            return Err(msg.to_string());
        }
    };

    let pg_db;
    match env::var_os(pg_db_env.clone()) {
        Some(x) => pg_db = x,
        None => {
            let msg = format!("No {} environment variable set!", pg_db_env);
            error!("{}", msg);
            return Err(msg.to_string());
        }
    };

    let pg_connection_str = format!(
        "postgresql://{}:{}@{}/{}",
        pg_username.to_str().unwrap(),
        pg_password.to_str().unwrap(),
        pg_uri.to_str().unwrap(),
        pg_db.to_str().unwrap()
    );

    match tokio_postgres::connect(&pg_connection_str, NoTls).await {
        Ok((pg_client, pg_connection)) => {
            // The connection object performs the actual communication with the database,
            // so spawn it off to run on its own.
            tokio::spawn(async move {
                if let Err(e) = pg_connection.await {
                    error!("connection error: {}", e);
                }
            });

            return Ok(pg_client);
        }
        Err(e) => {
            let msg = format!("Could not connect to PG SQL: {}", e);
            error!("{}", msg);
            return Err(msg.to_string());
        }
    }
}
