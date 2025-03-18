use crate::event::get_token::EventData;
use crate::functions::file_selector::file_dialog;

pub async fn get_data() {
    let output_path = file_dialog(false, None, None)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    for cc in ["jp", "tw", "en", "kr"] {
        let mut event = EventData {
            cc: Some(cc.to_string()),
            account_code: None,
            password: None,
            password_refresh_token: None,
            jwt_token: None,
        };

        for file in ["sale", "gatya", "item"] {
            let _ = event.to_file(output_path.clone(), cc, file).await;
        }
    }
}
