use clap::{Parser, Subcommand};
use smart_recommend_coin::capture::gmgn;
use smart_recommend_coin::capture::solscan;
use smart_recommend_coin::db;
use smart_recommend_coin::models;
use smart_recommend_coin::services::address_service;
use smart_recommend_coin::services::token_service;
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
    /// capture the token
    CaptureToken {
        #[arg(short, long, default_value_t = String::from("sol"))]
        chain: String,
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
                address_service::capture_save(capture).unwrap();
            })
            .await
            .expect("Failed to join blocking task");
            let response = address_service::read_capture_data();
            match response {
                Ok(data) => {
                    for (_i, item) in data.data.rank.into_iter().enumerate() {
                        if item.pnl_30d.unwrap_or_default() < 0.5 {
                            continue;
                        }
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
        Some(Commands::CaptureToken { chain }) => {
            println!("chain: {}", chain);
            let addresses = models::address::list_all(&pool).await?;

            for (_i, item) in addresses.into_iter().enumerate() {
                println!("Captureing address token: {}", item.address);
                let capture = solscan::SolscanCapture::new(item.address);
                let capture_data =
                    task::spawn_blocking(|| token_service::capture_get(capture).unwrap())
                        .await
                        .unwrap();
                for token in capture_data.data.tokens {
                    if token.token_address.is_none() {
                        continue;
                    }
                    let token_name = token.token_name.clone();
                    let token_symbol = token.token_symbol.clone();
                    let token_address = token.token_address.clone();
                    let params: models::token::AddTokenParams = token.into();
                    match models::token::add(&pool, params).await {
                        Ok(v) => {
                            println!("Added token: {}", v.token_name);
                        }
                        Err(e) => {
                            println!(
                                "Failed add token: {:?}, {}",
                                (token_symbol, token_name, token_address),
                                e
                            );
                        }
                    };
                }
            }
        }
        None => {}
    }
    Ok(())
}
