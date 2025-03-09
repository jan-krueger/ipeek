use actix_web::{dev, Error, HttpMessage};
use std::task::{Context, Poll};
use std::future::{ready, Ready};
use std::path::Path;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::http::Uri;

// Middleware to extract format and store it in request extensions
pub struct FormatMiddleware;

impl<S, B> dev::Transform<S, ServiceRequest> for FormatMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = FormatMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(FormatMiddlewareService { service }))
    }
}

// Middleware Service Implementation
pub struct FormatMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for FormatMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let path = req.path();
        let format = Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("plain")
            .to_ascii_lowercase();

        let clean_path = path.strip_suffix(&format!(".{}", format)).unwrap_or(path).to_string();

        req.extensions_mut().insert(format);

        if let Ok(updated_uri) = clean_path.parse::<Uri>() {
            req.match_info_mut().get_mut().update(&updated_uri);
        }

        self.service.call(req)
    }
}
