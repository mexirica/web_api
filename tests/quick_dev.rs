#![allow(unused)]

use anyhow::Result;
use serde_json::json;
#[tokio::test]
async fn quick_dev() -> Result<(), httpc_test::Error> {
    let hc_result = httpc_test::new_client("http://localhost:3000");

    let hc = hc_result.map_err(|err| {
        eprintln!("Erro ao criar cliente HTTP: {:?}", err);
        // Crie uma instância de httpc_test::Error diretamente
        httpc_test::Error::Generic("Erro ao criar cliente HTTP".to_string()) // Pode ser uma mensagem diferente, dependendo da implementação
            // outros campos da struct, se houver
    })?;

    hc.do_get("/").await?.print().await?;

    let req_login = hc.do_post("/api/login", json!({
        "username": "demo",
        "pwd": "helcome",
    }));

    req_login.await?.print().await?;

    Ok(())
}

// cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
