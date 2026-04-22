use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use infrastructure::auth::GoogleProvider;
use serde::{Deserialize, Serialize};

use app::auth::AuthService;
use infrastructure::user::PgUserRepo;

use crate::session;

pub struct AuthState {
    pub auth_service: Arc<AuthService<GoogleProvider, PgUserRepo>>,
    pub redirect_url: String,
}

#[derive(Serialize)]
pub struct LoginResponsePayload {
    redirect_url: String,
}

#[derive(Deserialize)]
pub struct CallbackParams {
    pub code: String,
    pub state: String,
}

pub async fn login(state: web::Data<AuthState>, session: Session) -> impl Responder {
    let (auth_url, csrf_state) = state.auth_service.get_login_url();

    if session.insert("oauth_state", &csrf_state).is_err() {
        return HttpResponse::InternalServerError().body("Session error");
    }

    let payload = LoginResponsePayload {
        redirect_url: auth_url,
    };

    match serde_json::to_string(&payload) {
        Ok(body) => HttpResponse::Ok().body(body),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

pub async fn callback(
    state: web::Data<AuthState>,
    params: web::Query<CallbackParams>,
    session: Session,
) -> impl Responder {
    let expected_state = match session.get::<String>("oauth_state") {
        Ok(Some(s)) => s,
        Ok(None) => return HttpResponse::BadRequest().body("Missing oauth_state"),
        Err(_) => return HttpResponse::BadRequest().body("Session error"),
    };
    session.remove("oauth_state");

    let user = match state
        .auth_service
        .handle_callback(&params.code, &params.state, &expected_state)
        .await
    {
        Ok(u) => u,
        Err(e) => return HttpResponse::Unauthorized().body(e.to_string()),
    };

    if session::set_user_id(&session, user.id).is_err() {
        return HttpResponse::InternalServerError().body("Session error");
    }

    HttpResponse::Found()
        .append_header(("Location", state.redirect_url.clone()))
        .finish()
}

pub async fn logout(session: Session) -> impl Responder {
    session::clear_session(&session);
    HttpResponse::NoContent().finish()
}
