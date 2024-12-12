use fantoccini::{Client, ClientBuilder, error::CmdError, wd::Capabilities};
use serde_json::json;
use tokio::process::{Child, Command};

pub async fn world() -> (Client, Child) {
    let driver = Command::new("geckodriver")
        .arg("--port")
        .arg("4444")
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("driver to start");

    let mut cap = Capabilities::new();
    cap.insert(
        "moz:firefoxOptions".to_string(),
        json!({
            "args": ["-headless"]
        }),
    );
    cap.insert(
        "goog:chromeOptions".to_string(),
        json!({
            "args": [
                "--headless",
                "--disable-gpu",
                "--disable-dev-shm-usage",
            ],
        }),
    );

    let client = ClientBuilder::native()
        .capabilities(cap)
        .connect("http://localhost:4444")
        .await
        .expect("client to connect");

    (client, driver)
}

pub async fn cleanup(client: Client, mut driver: Child) -> Result<(), CmdError> {
    client.close_window().await.unwrap();
    client.close().await.unwrap();
    driver.kill().await.unwrap();
    Ok(())
}
