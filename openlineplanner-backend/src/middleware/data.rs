use std::future::{ready, Ready};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, HttpMessage};
use futures_util::future::LocalBoxFuture;

use super::auth;
use crate::layers::LayersContainer;

#[derive(Default, Clone, Copy)]
pub struct UserDataExtraction;

impl<S, B> Transform<S, ServiceRequest> for UserDataExtraction
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = UserDataExtractionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(UserDataExtractionMiddleware { service }))
    }
}

pub struct UserDataExtractionMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for UserDataExtractionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        {
            let mut extensions = req.extensions_mut();
            let auth = extensions.get::<auth::Claims>().unwrap();
            let user_data = req
                .app_data::<LayersContainer>()
                .unwrap()
                .get_or_empty(&auth.sub);
            extensions.insert(user_data);
        };

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
