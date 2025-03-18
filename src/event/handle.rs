use crate::event::get_token::EventData;
use crate::functions::file_selector::file_dialog;

pub async fn get_data() -> Option<()> {
    let output_path: String = file_dialog(false, None, None)?.to_str()?.into();

    for cc in ["jp", "tw", "en", "kr"] {
        let mut event = EventData {
            account_code: None,
            password: None,
            password_refresh_token: None,
            jwt_token: None,
        };

        for file in ["sale", "gatya", "item"] {
            let _ = event.to_file(output_path.clone(), cc, file).await;
        }
    }

    Some(())
}
