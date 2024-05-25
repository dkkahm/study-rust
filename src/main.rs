use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _ = run().await?.await;
    Ok(())
}