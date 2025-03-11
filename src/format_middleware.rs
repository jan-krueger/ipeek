use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::http::Uri;
use actix_web::http::header::{ACCEPT};
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
        let (format, ext_str) = Format::from_path(path);

        // If no format was specified in the path, check Accept header
        let format = if format == Format::Plain {
            if let Some(accept_header) = req.headers().get(ACCEPT) {
                if let Ok(accept) = accept_header.to_str() {
                    Format::from_accept_header(accept)
                } else {
                    Format::Plain
                }
            } else {
                Format::Plain
            }
        } else {
            format
        };

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
    Yml,
    Msgpack,
    Txt,
    Plain,
}

impl FromStr for Format {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "json" => Ok(Format::Json),
            "xml" => Ok(Format::Xml),
            "csv" => Ok(Format::Csv),
            "yaml" | "yml" => Ok(Format::Yml),
            "msgpack" => Ok(Format::Msgpack),
            "txt" => Ok(Format::Txt),
            _ => Ok(Format::Plain),
        }
    }
}

impl Format {
    fn from_accept_header(accept: &str) -> Format {
        for media_range in accept.split(',') {
            let media_type = media_range.split(';').next().unwrap_or("").trim();
            match media_type {
                "application/json" => return Format::Json,
                "application/xml" | "text/xml" => return Format::Xml,
                "text/csv" => return Format::Csv,
                "application/yaml" | "text/yaml" => return Format::Yml,
                "application/msgpack" | "application/x-msgpack" => return Format::Msgpack,
                "text/plain" => return Format::Txt,
                _ => continue,
            }
        }
        Format::Plain
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Format::Json => "json",
            Format::Xml => "xml",
            Format::Csv => "csv",
            Format::Yml => "yml",
            Format::Msgpack => "msgpack",
            Format::Txt => "txt",
            Format::Plain => "",
        };
        write!(f, "{}", s)
    }
}

impl Format {
    pub fn from_path(path: &str) -> (Format, &str) {
        let file_name = match Path::new(path).file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => return (Format::Plain, ""),
        };

        if let Some(dot_index) = file_name.rfind('.') {
            if dot_index < file_name.len() - 1 {
                let ext = &file_name[dot_index + 1..];
                (Format::from_str(ext).unwrap(), ext)
            } else {
                (Format::Plain, "")
            }
        } else {
            (Format::Plain, "")
        }
    }
}
