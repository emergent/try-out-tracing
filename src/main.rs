use anyhow::Result;
use tracing::{error, event, info, Level};

#[derive(Debug)]
struct MyStruct {
    name: String,
    value: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let my = MyStruct {
        name: "aaa".into(),
        value: 42,
    };

    info!(name = ?my.name, value = ?my.value);

    if let Err(e) = run(my.value) {
        error!(error = %e, "error ga deta yo");
    };

    println!("Hello, world!");
    Ok(())
}

#[tracing::instrument(fields(x2 = %(x*x)))]
fn run(x: i32) -> Result<()> {
    event!(Level::INFO, x = x);
    if x < 30 {
        Ok(())
    } else {
        let e = anyhow::anyhow!("error-occurred");
        Err(e)
    }
}
