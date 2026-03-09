use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{auth::providers::OAuthProvider, db::insert_user, session};

pub struct AuthState {
    pub provider: Arc<dyn OAuthProvider>,
    pub db: PgPool,
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
    let (auth_url, csrf_state) = state.provider.authorization_url();

    if session.insert("oauth_state", &csrf_state).is_err() {
        return HttpResponse::InternalServerError().body("Session error");
    }
    println!("session inserted oauth_state {}", csrf_state);

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
    println!("session : {:?}", session.status());
    let expected_state = match session.get::<String>("oauth_state") {
        Ok(Some(s)) => s,
        Ok(None) => return HttpResponse::BadRequest().body("Missing oauth_state none"),
        Err(e) => {
            println!("session error: {:?}", e);
            return HttpResponse::BadRequest().body("Missing oauth_state none");
        }
    };
    session.remove("oauth_state");

    let provided_user = match state
        .provider
        .exchange_code(&params.code, &params.state, &expected_state)
        .await
    {
        Ok(u) => u,
        Err(e) => {
            println!("error: {:?}", e);
            return HttpResponse::Unauthorized().body(e.to_string());
        }
    };

    let user = match insert_user(&state.db, provided_user.email, provided_user.name).await {
        Ok(u) => u,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    };

    if session::set_user_id(&session, user.id).is_err() {
        return HttpResponse::InternalServerError().body("Session error");
    }

    HttpResponse::Found()
        .append_header(("Location", "http://localhost:3000/"))
        .finish()
}

pub async fn logout(session: Session) -> impl Responder {
    session::clear_session(&session);

    HttpResponse::NoContent()
}
