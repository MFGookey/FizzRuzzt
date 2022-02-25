use tokio::signal;

mod game;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {fizz_ruzzt::run().await});

    match signal::ctrl_c().await {
      Ok(()) => {},
      Err(err) => {eprintln!("Unable to listen to shutdown signal {}", err); }
    }

    handle.abort();
}