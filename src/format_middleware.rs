use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::http::Uri;
use actix_web::{dev, Error, HttpMessage};
use std::fmt;
use std::future::{ready, Ready};
use std::path::Path;
use std::str::FromStr;
use std::task::{Context, Poll};

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
        let ext_str = Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("plain");

        let format = Format::from_str(ext_str).unwrap();

        let clean_path = path
            .strip_suffix(&format!(".{}", ext_str))
            .unwrap_or(path)
            .to_string();

        req.extensions_mut().insert(format);

        if let Ok(updated_uri) = clean_path.parse::<Uri>() {
            req.match_info_mut().get_mut().update(&updated_uri);
        }

        self.service.call(req)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
    Json,
    Xml,
    Csv,
    Yaml,
    Msgpack,
    Plain,
}

impl FromStr for Format {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "xml" => Ok(Format::Xml),
            "csv" => Ok(Format::Csv),
            "yaml" => Ok(Format::Yaml),
            "msgpack" => Ok(Format::Msgpack),
            _ => Ok(Format::Plain),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Format::Json => "json",
            Format::Xml => "xml",
            Format::Csv => "csv",
            Format::Yaml => "yaml",
            Format::Msgpack => "msgpack",
            Format::Plain => "",
        };
        write!(f, "{}", s)
    }
}
