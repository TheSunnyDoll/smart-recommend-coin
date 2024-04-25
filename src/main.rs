use axum::http::HeaderValue;
use axum::http::Method;
use axum::routing::get;
use axum::Router;
use clap::{Parser, Subcommand};
use smart_recommend_coin::capture::gmgn;
use smart_recommend_coin::capture::solscan;
use smart_recommend_coin::db;
use smart_recommend_coin::models;
use smart_recommend_coin::services::address_service;
use smart_recommend_coin::services::token_service;
use tokio::task;
use tower_http::cors::CorsLayer;
use tracing::warn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
    StatToken {},
    ApiServer {},
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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
                                if let Some(vv) = v {
                                    println!("Added address: {}", vv.address);
                                }
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
                let capture_resp =
                    task::spawn_blocking(|| token_service::capture_get(capture).unwrap()).await;
                match capture_resp {
                    Ok(capture_data) => {
                        for token in capture_data.data.tokens {
                            if token.token_address.is_none() {
                                continue;
                            }
                            let token_name = token.token_name.clone();
                            let token_symbol = token.token_symbol.clone();
                            let token_address = token.token_address.clone();
                            let params: models::address_token::AddTokenParams = token.into();
                            match models::address_token::add(&pool, params).await {
                                Ok(v) => {
                                    if let Some(vv) = v {
                                        println!("Added token: {}", vv.token_name);
                                    }
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
                    Err(e) => {
                        warn!("capture token: {}", e)
                    }
                }
            }
        }
        Some(Commands::StatToken {}) => {
            token_service::stat_token_holder(pool.clone())
                .await
                .unwrap();
        }
        Some(Commands::ApiServer {}) => {
            let app = Router::new()
                .route("/api/tokens/recommend", get(token_service::tokens))
                .layer(
                    CorsLayer::new()
                        .allow_origin("*".parse::<HeaderValue>().unwrap())
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::PUT,
                            Method::OPTIONS,
                            Method::DELETE,
                        ]),
                )
                .with_state(pool.clone());

            let listener = tokio::net::TcpListener::bind("127.0.0.1:7000")
                .await
                .unwrap();
            tracing::info!("listening on {}", listener.local_addr().unwrap());
            axum::serve(listener, app).await.unwrap();
        }
        None => {}
    }
    Ok(())
}
