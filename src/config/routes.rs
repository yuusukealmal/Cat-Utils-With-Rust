pub const EVENT_CREATE_ACCOUNT: &str =
    "https://nyanko-backups.ponosgames.com/?action=createAccount&referenceId=";
pub const EVENT_USER_AUTH: &str = "https://nyanko-auth.ponosgames.com/v1/users";
pub const EVENT_USER_TOKEN: &str = "https://nyanko-auth.ponosgames.com/v1/tokens";

#[allow(non_snake_case)]
pub fn EVENT_FILE(cc: &str, file: &str, jwt: &str) -> String {
    format!(
        "https://nyanko-events.ponosgames.com/battlecats{}_production/{}.tsv?jwt={}",
        cc, file, jwt
    )
}

#[allow(non_snake_case)]
pub fn PLACEMENT_JSON(cc: &str) -> String {
    format!(
        "https://nyanko-events.ponosgames.com/control/placement/battlecats{}/event.json",
        cc
    )
}

#[allow(non_snake_case)]
pub fn PLACEMENT_PNG(cc: &str, uuid: &str) -> String {
    format!(
        "https://ponosgames.com/information/appli/battlecats/placement{}/notice_{}.png",
        cc, uuid
    )
}

#[allow(non_snake_case)]
pub fn GET_SAVE_FILE(transfer: &str) -> String {
    format!(
        "https://nyanko-save.ponosgames.com/v2/transfers/{}/reception",
        transfer
    )
}

pub const CLOUDFRONT_SIGN_URL: &str = "https://nyanko-assets.ponosgames.com/*";

#[allow(non_snake_case)]
pub fn SERVER_ASSETS_ZIP(cc: &str, version: &str) -> String {
    format!(
        "https://nyanko-assets.ponosgames.com/iphone/{}/download/{}.zip",
        cc, version
    )
}

#[allow(non_snake_case)]
pub fn TRACK_UNITBUY(cc: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/yuusukealmal/Cat-data/refs/heads/main/Data/local/jp.co.ponos.battlecats{}/DataLocal/unitbuy.csv",
        cc
    )
}

#[allow(non_snake_case)]
pub fn TRACK_GATYA_SET(cc: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/yuusukealmal/Cat-data/refs/heads/main/Data/local/jp.co.ponos.battlecats{}/DataLocal/GatyaDataSetR1.csv",
        cc
    )
}

#[allow(non_snake_case)]
pub fn TRACK_EVENT_DATA(cc: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/yuusukealmal/Cat-data/refs/heads/main/Data/event/{}_gatya.tsv",
        cc.to_uppercase()
    )
}

#[allow(non_snake_case)]
pub fn TRACK_UNIT_EXPLANATION(cc: &str, id: u32) -> String {
    let cc_display = if cc == "jp" { "ja" } else { cc };
    format!(
        "https://raw.githubusercontent.com/yuusukealmal/Cat-data/refs/heads/main/Data/local/jp.co.ponos.battlecats{}/resLocal/Unit_Explanation{}_{}.csv",
        cc, id, cc_display
    )
}
