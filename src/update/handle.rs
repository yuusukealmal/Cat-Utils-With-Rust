use crate::event;
use crate::functions::git::{commit_or_push, Method};
use crate::placement;

pub async fn update() -> Result<(), Box<dyn std::error::Error>> {
    event::handle::get_event_data(Some(true)).await?;
    commit_or_push(Method::PUSH, None)?;
    placement::handle::get_announcement(Some(true)).await?;
    commit_or_push(Method::PUSH, None)?;
    super::update_xapk::update().await?;
    commit_or_push(Method::PUSH, None)?;

    Ok(())
}
