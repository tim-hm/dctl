use std::error::Error;

use hyper::StatusCode;

#[tokio::test]
async fn health_check() -> Result<(), Box<dyn Error>> {
    let host = utils::constants::DEV_ADDRESS;
    let port = utils::env::get_port();
    let url = format!("{host}:{port}");

    let resp = reqwest::get(url).await?;
    let status = resp.status();

    assert_eq!(status.as_str(), StatusCode::OK.as_str());

    Ok(())
}
