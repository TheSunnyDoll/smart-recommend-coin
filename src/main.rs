use chrono::Local;
use chrono::TimeZone;
use clap::{Parser, Subcommand};
use smart_recommend_coin::capture_address;
use smart_recommend_coin::db;
use smart_recommend_coin::gmgn;
use smart_recommend_coin::models;
use tokio::task;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// capture the top addresses
    CaptureAddress {
        #[arg(short, long, default_value_t = String::from("sol"))]
        chain: String,
        #[arg(short, long, default_value_t = 30)]
        days: i8,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    db::init().await.unwrap();
    let pool = db::get_pool().await.unwrap();

    match &cli.command {
        Some(Commands::CaptureAddress { chain, days }) => {
            println!("chain: {}, days: {}", chain, days);
            let capture = gmgn::GmgnCapture::new(chain.to_string(), *days);
            task::spawn_blocking(|| {
                capture_address::perform_capture(capture).unwrap();
            })
            .await
            .expect("Failed to join blocking task");
            let response = capture_address::read();
            match response {
                Ok(data) => {
                    for (_i, item) in data.data.rank.into_iter().enumerate() {
                        // println!(
                        //     "{} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                        //     i,
                        //     item.address,
                        //     item.sol_balance.unwrap(),
                        //     item.pnl_1d.unwrap_or_default(),
                        //     item.pnl_7d.unwrap_or_default(),
                        //     item.pnl_30d.unwrap_or_default(),
                        //     item.realized_profit.unwrap(),
                        //     item.txs_30d,
                        //     item.avg_hold_time.unwrap_or_default() / 3600.0,
                        //     Local.timestamp_opt(item.last_active, 0).unwrap()
                        // );
                        let address = item.address.clone();
                        let mut params: models::address::AddAddressParams = item.into();
                        params.source = "gmgn".to_string();
                        match models::address::add(&pool, params).await {
                            Ok(v) => {
                                println!("Added address: {}", v.address);
                            }
                            Err(e) => {
                                println!("Failed add address: {}, {}", address, e);
                            }
                        };
                    }
                }
                Err(e) => {
                    println!("{:?}", e)
                }
            }
        }
        None => {}
    }
    Ok(())
}
