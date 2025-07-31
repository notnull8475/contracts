use actix_web::cookie::{Cookie, SameSite, time};
use actix_web::{
    Error, HttpMessage,
    dev::{ServiceRequest, ServiceResponse, Transform},
};
use futures_util::future::{LocalBoxFuture, Ready, ok};
use rand::distr::Alphanumeric;
use rand::{Rng, rng};
use std::task::{Context, Poll};

// Тип для хранения guest_id в Extensions
#[derive(Clone)]
pub struct GuestId(pub String);

// Middleware
pub struct GuestCookie;

impl<S> Transform<S, ServiceRequest> for GuestCookie
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = GuestCookieMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(GuestCookieMiddleware { service })
    }
}

pub struct GuestCookieMiddleware<S> {
    service: S,
}

impl<S> actix_web::dev::Service<ServiceRequest> for GuestCookieMiddleware<S>
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Проверяем наличие guest_id в куках
        let guest_id = match req.cookie("guest_id") {
            Some(c) => c.value().to_string(),
            None => {
                // Генерируем уникальный 16-символьный строковый ID
                let id: String = rng()
                    .sample_iter(&Alphanumeric)
                    .take(16)
                    .map(char::from)
                    .collect();
                id
            }
        };

        // Сохраняем guest_id в Extensions для использования в handlers
        req.extensions_mut()
            .insert::<GuestId>(GuestId(guest_id.clone()));

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;

            // Если не было guest_id — ставим куку
            if res.request().cookie("guest_id").is_none() {
                res.response_mut()
                    .add_cookie(
                        &Cookie::build("guest_id", guest_id)
                            .path("/")
                            .max_age(time::Duration::days(365))
                            .same_site(SameSite::Lax)
                            .http_only(true)
                            .secure(true) // только через HTTPS
                            .finish(),
                    )
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            }

            Ok(res)
        })
    }
}
