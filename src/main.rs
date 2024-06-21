use async_std::net::TcpStream;
use dotenv::dotenv;
use std::env;
use tiberius::{Client, Config};

struct Vehicle {
    manufacturing_year: String,
    make: String,
    model: String,
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let vehicles = get_vehicles().await?;

    for vehicle in vehicles {
        println!(
            "{} {} {}",
            vehicle.manufacturing_year, vehicle.make, vehicle.model
        );
    }

    Ok(())
}

async fn get_vehicles() -> anyhow::Result<Vec<Vehicle>> {
    let connection_string = env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set");

    let config = Config::from_ado_string(&connection_string)?;
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp).await?;

    let mut vehicles = Vec::<Vehicle>::new();

    let rows = client
        .query(
            "SELECT Manufacturing_Year, Make, Model FROM vehicles",
            &[&1i32],
        )
        .await?
        .into_first_result()
        .await?;

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

    Ok(vehicles)
}
