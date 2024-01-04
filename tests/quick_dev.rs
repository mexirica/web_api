#![allow(unused)]

use anyhow::Result;
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000");

    hc.unwrap().do_get("/").await?.print().await?;

    Ok(())
}
// cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
