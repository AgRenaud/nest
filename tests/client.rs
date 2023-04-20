#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn client() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;

    hc.do_get("/").await?.print().await?;

    // hc.do_get("/simple").await?.print().await?;

    Ok(())
}
