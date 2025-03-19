pub async fn get_placement(cc: &str) -> reqwest::Result<String> {
    let cc_suffix = if cc == "jp" { "" } else { cc };
    let result = reqwest::get(format!("https://nyanko-events.ponosgames.com/control/placement/battlecats{cc_suffix}/event.json").as_str()).await?.text().await?;

    Ok(result)
}
