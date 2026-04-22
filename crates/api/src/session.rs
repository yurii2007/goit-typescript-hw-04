use actix_session::Session;
use uuid::Uuid;

const USER_ID_KEY: &str = "user_id";

pub fn set_user_id(session: &Session, user_id: Uuid) -> Result<(), String> {
    session.insert(USER_ID_KEY, user_id).map_err(|e| e.to_string())
}

pub fn get_user_id(session: &Session) -> Option<Uuid> {
    session.get::<Uuid>(USER_ID_KEY).ok().flatten()
}

pub fn clear_session(session: &Session) {
    session.purge();
}
