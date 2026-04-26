use actix_session::SessionExt;
use actix_web::{
  Error, HttpResponse,
  body::EitherBody,
  dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use std::rc::Rc;
use uuid::Uuid;

pub struct RequireAuth;

impl<S, B> Transform<S, ServiceRequest> for RequireAuth
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Transform = RequireAuthMiddleware<S>;
  type InitError = ();
  type Future = Ready<Result<Self::Transform, Self::InitError>>;

  fn new_transform(&self, service: S) -> Self::Future {
    ok(RequireAuthMiddleware {
      service: Rc::new(service),
    })
  }
}

pub struct RequireAuthMiddleware<S> {
  service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequireAuthMiddleware<S>
where
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: 'static,
{
  type Response = ServiceResponse<EitherBody<B>>;
  type Error = Error;
  type Future = LocalBoxFuture<'static, Result<ServiceResponse<EitherBody<B>>, Error>>;

  forward_ready!(service);

  fn call(&self, req: ServiceRequest) -> Self::Future {
    let service = self.service.clone();

    Box::pin(async move {
      let session = req.get_session();
      let user_id = session.get::<Uuid>("user_id").ok().flatten();

      if user_id.is_none() {
        let (req, _payload) = req.into_parts();
        let response = HttpResponse::Unauthorized()
          .body("Not authenticated")
          .map_into_right_body();
        return Ok(ServiceResponse::new(req, response));
      }

      service
        .call(req)
        .await
        .map(ServiceResponse::map_into_left_body)
    })
  }
}
