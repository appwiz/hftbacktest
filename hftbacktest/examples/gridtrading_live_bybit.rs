use algo::gridtrading;
use hftbacktest::{
    live::{LiveBot, LoggingRecorder},
    prelude::{Bot, ErrorKind, HashMapMarketDepth},
};
use tracing::error;

mod algo;

const ORDER_PREFIX: &str = "prefix";

fn prepare_live() -> LiveBot<HashMapMarketDepth> {
    let mut hbt = LiveBot::builder()
        .add("bybit-futures", "BTCUSDT", 0.1, 0.001)
        .depth(|asset| HashMapMarketDepth::new(asset.tick_size, asset.lot_size))
        .error_handler(|error| {
            match error.kind {
                ErrorKind::ConnectionInterrupted => {
                    error!("ConnectionInterrupted");
                }
                ErrorKind::CriticalConnectionError => {
                    error!("CriticalConnectionError");
                }
                ErrorKind::OrderError => {
                    let error = error.value();
                    error!(?error, "OrderError");
                }
                ErrorKind::Custom(errno) => {
                    error!(%errno, "custom");
                }
            }
            Ok(())
        })
        .build()
        .unwrap();

    hbt.run().unwrap();
    hbt
}

fn main() {
    tracing_subscriber::fmt::init();

    let mut hbt = prepare_live();

    let relative_half_spread = 0.0001;
    let relative_grid_interval = 0.0001;
    let grid_num = 2;
    let min_grid_step = 0.1; // tick size
    let skew = relative_half_spread / grid_num as f64;
    let order_qty = 0.001;
    let max_position = grid_num as f64 * order_qty;

    let mut recorder = LoggingRecorder::new();
    gridtrading(
        &mut hbt,
        &mut recorder,
        relative_half_spread,
        relative_grid_interval,
        grid_num,
        min_grid_step,
        skew,
        order_qty,
        max_position,
    )
    .unwrap();
    hbt.close().unwrap();
}
