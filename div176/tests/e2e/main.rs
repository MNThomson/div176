use fantoccini::error::CmdError;
use pretty_assertions::assert_eq;

mod common;
use common::*;

#[tokio::test]
#[ignore = "requires geckodriver"]
async fn tst() -> Result<(), CmdError> {
    let (c, d) = world().await;

    let _ = c.goto("https://example.com").await?;
    assert_eq!("Example Domain", c.title().await?);

    cleanup(c, d).await
}
