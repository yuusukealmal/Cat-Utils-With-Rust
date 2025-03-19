pub async fn get_placement(cc: &str) -> reqwest::Result<String> {
    let cc_suffix = if cc == "jp" { "" } else { cc };
    let url = format!(
        "https://nyanko-events.ponosgames.com/control/placement/battlecats{cc_suffix}/event.json"
    );

    reqwest::get(&url).await?.text().await
}
