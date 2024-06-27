use anyhow::Error;
use async_std::net::TcpStream;
use axum::{body::Body, response::IntoResponse, response::Json, routing::get, Router};
use dotenv::dotenv;
use serde::Serialize;
use serde_json::{json, Value};
use std::env;
use tiberius::{Client, Config};

#[derive(Debug, Serialize)]
struct Vehicle {
    manufacturing_year: String,
    make: String,
    model: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/vehicles", get(get_vehicles));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_vehicles() -> impl IntoResponse {
    match fetch_vehicles().await {
        Ok(json) => Ok(json),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string(),
        )),
    }
}

async fn fetch_vehicles() -> Result<Json<Value>, Error> {
    let connection_string = env::var("CONNECTION_STRING")
        .expect("CONNECTION_STRING must be set in .env file or environment variables");

    let config =
        Config::from_ado_string(&connection_string).expect("Failed to parse connection string");
    let tcp = TcpStream::connect(config.get_addr())
        .await
        .expect("Failed to connect to SQL Server");

    tcp.set_nodelay(true).expect("Failed to set TCP_NODELAY");

    let mut client = Client::connect(config, tcp)
        .await
        .expect("Failed to connect to SQL Server");

    let mut vehicles = Vec::<Vehicle>::new();

    let rows = client
        .query("EXEC [dbo].[test] @mode = 1", &[&1i32])
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

    Ok(Json(json!({ "data": vehicles })))
}
