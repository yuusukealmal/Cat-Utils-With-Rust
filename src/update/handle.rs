use crate::event;
use crate::placement;

pub async fn update() -> Result<(), Box<dyn std::error::Error>> {
    event::handle::get_event_data(Some(true)).await?;
    placement::handle::get_announcement(Some(true)).await?;
    super::update_xapk::update().await?;

    Ok(())
}
