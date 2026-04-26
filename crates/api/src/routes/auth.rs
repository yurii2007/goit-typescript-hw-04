use std::sync::Arc;

use actix_session::Session;
use actix_web::{HttpResponse, Responder, cookie::Cookie, web};
use infrastructure::auth::GoogleProvider;
use serde::Deserialize;

use app::auth::AuthService;
use infrastructure::user::PgUserRepo;

use crate::{
  features::user::{ApiResponseUser, AuthStatusData, AuthStatusResponse},
  session,
};

pub struct AuthState {
  pub auth_service: Arc<AuthService<GoogleProvider, PgUserRepo>>,
  pub redirect_url: String,
}

#[derive(Deserialize)]
pub struct CallbackParams {
  pub code: String,
  pub state: String,
}

pub async fn get_current_user(state: web::Data<AuthState>, session: Session) -> HttpResponse {
  let Some(user_id) = session::get_user_id(&session) else {
    return HttpResponse::Unauthorized().json(AuthStatusResponse {
      data: AuthStatusData { user: None },
      status: 401,
    });
  };

  match state.auth_service.get_current_user(user_id).await {
    Ok(Some(user)) => HttpResponse::Ok().json(AuthStatusResponse {
      data: AuthStatusData {
        user: Some(ApiResponseUser::from(user)),
      },
      status: 200,
    }),
    Ok(None) => HttpResponse::NotFound().json(AuthStatusResponse {
      data: AuthStatusData { user: None },
      status: 404,
    }),
    Err(_) => HttpResponse::Unauthorized().json(AuthStatusResponse {
      data: AuthStatusData { user: None },
      status: 401,
    }),
  }
}

pub async fn login(state: web::Data<AuthState>, session: Session) -> impl Responder {
  let (auth_url, csrf_state) = state.auth_service.get_login_url();

  if session.insert("oauth_state", &csrf_state).is_err() {
    return HttpResponse::InternalServerError().body("Session error");
  }

  HttpResponse::Found()
    .append_header(("Location", auth_url))
    .finish()
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

pub async fn logout(session: Session) -> HttpResponse {
  session::clear_session(&session);

  let removal_cookie = Cookie::build("id", "")
    .path("/")
    .http_only(true)
    .secure(true)
    .same_site(actix_web::cookie::SameSite::None)
    .expires(actix_web::cookie::time::OffsetDateTime::UNIX_EPOCH)
    .finish();

  HttpResponse::NoContent().cookie(removal_cookie).finish()
}
