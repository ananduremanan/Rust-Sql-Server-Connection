use async_std::net::TcpStream;
use dotenv::dotenv;
use std::env;
use tiberius::{Client, Config};
use warp::{Filter, Rejection, Reply};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Vehicle {
    manufacturing_year: String,
    make: String,
    model: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let get_vehicles = warp::path("vehicles")
        .and(warp::get())
        .and_then(get_vehicles_handler);

    let routes = get_vehicles;

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}

async fn get_vehicles_handler() -> Result<impl Reply, Rejection> {
    let connection_string = env::var("CONNECTION_STRING")
        .expect("CONNECTION_STRING must be set in .env file or environment variables");

    let config = Config::from_ado_string(&connection_string)
        .expect("Failed to parse connection string");
    let tcp = TcpStream::connect(config.get_addr()).await
        .expect("Failed to connect to SQL Server");
    tcp.set_nodelay(true)
        .expect("Failed to set TCP_NODELAY");

    let mut client = Client::connect(config, tcp).await
        .expect("Failed to connect to SQL Server");

    let mut vehicles = Vec::<Vehicle>::new();

    let rows = client
        .query(
            "SELECT Manufacturing_Year, Make, Model FROM [GRAMPRO\\40412].vehicles",
            &[&1i32],
        )
        .await
        .expect("Failed to execute query")
        .into_first_result()
        .await
        .expect("Failed to get first result");

    for row in rows {
        let manufacturing_year: &str = row.get("Manufacturing_Year").unwrap();
        let make: &str = row.get("Make").unwrap();
        let model: &str = row.get("Model").unwrap();

        vehicles.push(Vehicle {
            manufacturing_year: manufacturing_year.to_string(),
            make: make.to_string(),
            model: model.to_string(),
        });
    }

    Ok(warp::reply::json(&vehicles))
}