pub async fn get_placement(cc: &str) -> reqwest::Result<String> {
    let cc_suffix = if cc == "jp" { "" } else { cc };
    let url = format!(
        "https://nyanko-events.ponosgames.com/control/placement/battlecats{cc_suffix}/event.json"
    );

    reqwest::get(&url).await?.text().await
}

pub async fn get_pictures(cc_suffix: &str, uuid: &str) -> reqwest::Result<bytes::Bytes> {
    let url = String::from(format!(
        "https://ponosgames.com/information/appli/battlecats/placement{}/notice_{}.png",
        cc_suffix, uuid
    ));

    reqwest::get(&url).await.unwrap().bytes().await
}
