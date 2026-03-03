use thirtyfour::prelude::*;

async fn setup_driver() -> WebDriverResult<WebDriver> {
    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;
    WebDriver::new("http://localhost:4444", caps).await
}

#[tokio::test]
#[ignore = "Requires geckodriver running on port 4444"]
async fn test_homepage_loads() -> WebDriverResult<()> {
    let driver = setup_driver().await?;

    driver.goto("http://localhost:8080").await?;

    let title = driver.title().await?;
    assert!(title.contains("做甚麼都有如神助") || title.contains("Developer"));

    driver.quit().await?;
    Ok(())
}

#[tokio::test]
#[ignore = "Requires geckodriver running on port 4444"]
async fn test_navigation_to_base64() -> WebDriverResult<()> {
    let driver = setup_driver().await?;

    driver.goto("http://localhost:8080").await?;

    let base64_link = driver.query(By::Css("a[href*='base64']")).first().await?;
    base64_link.click().await?;

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let url = driver.current_url().await?.to_string();
    assert!(url.contains("base64"));

    driver.quit().await?;
    Ok(())
}

#[tokio::test]
#[ignore = "Requires geckodriver running on port 4444"]
async fn test_base64_encode() -> WebDriverResult<()> {
    let driver = setup_driver().await?;

    driver.goto("http://localhost:8080/base64").await?;

    let textareas = driver.find_all(By::Css("textarea")).await?;
    assert!(
        textareas.len() >= 2,
        "Should have input and output textareas"
    );

    textareas[0].send_keys("Hello World").await?;

    let encode_btn = driver.find(By::Css("button")).await?;
    encode_btn.click().await?;

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let output = textareas[1].text().await?;
    assert!(
        !output.is_empty(),
        "Output should not be empty after encoding"
    );

    driver.quit().await?;
    Ok(())
}

#[tokio::test]
#[ignore = "Requires geckodriver running on port 4444"]
async fn test_theme_toggle() -> WebDriverResult<()> {
    let driver = setup_driver().await?;

    driver.goto("http://localhost:8080").await?;

    let theme_btn = driver.find(By::Css(".theme-switch")).await?;
    theme_btn.click().await?;

    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let layout = driver.find(By::Css(".layout")).await?;
    let class = layout.class_name().await?;
    assert!(
        class.unwrap_or_default().contains("light"),
        "Should have light theme"
    );

    driver.quit().await?;
    Ok(())
}
