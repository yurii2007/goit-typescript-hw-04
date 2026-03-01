use actix_session::Session;
use uuid::Uuid;

const USER_ID_KEY: &str = "user_id";

pub fn set_user_id(session: &Session, user_id: Uuid) -> anyhow::Result<()> {
    session.insert(USER_ID_KEY, user_id)?;
    Ok(())
}

pub fn get_user_id(session: &Session) -> anyhow::Result<Option<Uuid>> {
    let user_id = session.get::<Uuid>(USER_ID_KEY).ok().flatten();

    Ok(user_id)
}

pub fn clear_session(session: &Session) {
    session.purge();
}
