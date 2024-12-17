async fn fetch_crypto_prices(tx: Arc<broadcast::Sender<String>>) {
    let client = reqwest::Client::new();
    let crypto_api = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";

    loop {
        if let Ok(response) = client.get(crypto_api).send().await {
            if let Ok(prices) = response.json::<serde_json::Value>().await {
                let btc_price = prices["bitcoin"]["usd"].as_f64().unwrap_or_default();
                let eth_price = prices["ethereum"]["usd"].as_f64().unwrap_or_default();
                let update = format!(
                    r#"{{"bitcoin": {}, "ethereum": {}}}"#,
                    btc_price, eth_price
                );

                // Broadcast the update to all connected clients
                let _ = tx.send(update);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}