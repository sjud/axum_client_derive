use std::borrow::Borrow;
use std::net::{SocketAddr, TcpListener};
use axum::headers::{Authorization, Error, Header, UserAgent};
use axum::http::StatusCode;
use axum::{headers, Json, Router, TypedHeader};
use axum::body::{Body, BoxBody, HttpBody};
use axum::extract::State;
use axum::headers::authorization::Credentials;
use axum::routing::get;
use http::header::HeaderName;
use http::{HeaderValue, Request};
use reqwest::Client;
use axum_client_derive::reqwest_fn;
use axum::routing::post;
use bytes::Bytes;
use serde::{Serialize,Deserialize};
use axum::routing::put;
pub static JWT : HeaderName = HeaderName::from_static("jwt");



#[derive(Clone)]
pub struct ArbState;

pub struct Jwt(String);
impl Header for Jwt {
    fn name() -> &'static HeaderName {
        &JWT
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
        where
            I: Iterator<Item = &'i HeaderValue>,{
        let value = values
            .next()
            .ok_or_else(headers::Error::invalid)?;
        Ok(Self(value.to_str().unwrap().to_string()))
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let value = HeaderValue::from_str(&self.0).unwrap();
        values.extend(std::iter::once(value));
    }
}
#[reqwest_fn("get")]
async fn type_header_handler(
    jwt: TypedHeader<Jwt>,
    state:State<ArbState>,
) -> StatusCode {
    println!("{}",jwt.0.0);
    StatusCode::OK
}

#[tokio::test]
async fn test_type_header_handler() {
    let router = Router::with_state(ArbState)
        .route("/hello",
               get(type_header_handler));
    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();

    let route = format!("http://{}{}",addr,"/hello");

    eprintln!("{}",route);
    assert_eq!(
        client_derive_reqwest_fn_type_header_handler::type_header_handler(
            &client,
            route,
            Jwt("Hello world.".into())
        ).await.unwrap().status(),
        StatusCode::OK
    );
}
#[reqwest_fn("post")]
async fn body_string_handler(
    state:State<String>,
    body: String
) -> StatusCode {
    assert_eq!(body,state.0);
    StatusCode::OK
}
#[tokio::test]
async fn test_body_string_handler() {
    let router = Router::with_state(String::from("Hello World."))
        .route("/hello",
               post(body_string_handler));
    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();

    let route = format!("http://{}{}",addr,"/hello");

    eprintln!("{}",route);
    assert_eq!(
        client_derive_reqwest_fn_body_string_handler::body_string_handler(
            &client,
            route,
            "Hello World.".into()
        ).await.unwrap().status(),
        StatusCode::OK
    );
}
#[reqwest_fn("post")]
async fn body_bytes_handler(
    state:State<Bytes>,
    body: Bytes
) -> StatusCode {
    assert_eq!(body,state.0);
    StatusCode::OK
}
#[tokio::test]
async fn test_body_bytes_handler() {
    let router = Router::with_state(Bytes::from("Hello World."))
        .route("/hello",
               post(body_bytes_handler));
    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();

    let route = format!("http://{}{}",addr,"/hello");

    eprintln!("{}",route);
    assert_eq!(
        client_derive_reqwest_fn_body_bytes_handler::body_bytes_handler(
            &client,
            route,
            Bytes::from("Hello World.")
        ).await.unwrap().status(),
        StatusCode::OK
    );
}

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq)]
pub struct Email {
    email: String,
}
#[reqwest_fn("post")]
async fn body_json_handler(
    state:State<Email>,
    body: Json<Email>,
) -> StatusCode {
    assert_eq!(body.0,state.0);
    StatusCode::OK
}
#[tokio::test]
async fn test_body_json_handler() {
    let router = Router::with_state(Email{email:"hello@world.test".into()})
        .route("/hello",
               post(body_json_handler));
    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();

    let route = format!("http://{}{}",addr,"/hello");

    eprintln!("{}",route);
    assert_eq!(
        client_derive_reqwest_fn_body_json_handler::body_json_handler(
            &client,
            route,
            &Email{email:"hello@world.test".into()}
        ).await.unwrap().status(),
        StatusCode::OK
    );
}

#[reqwest_fn("post")]
async fn body_request_handler<B:HttpBody>(
    body: Request<B>,
) -> StatusCode {
    StatusCode::OK
}
#[tokio::test]
async fn test_body_request_handler() {
    let router = Router::new()
        .route("/hello",
               post(body_request_handler));
    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();

    let route = format!("http://{}{}",addr,"/hello");

    eprintln!("{}",route);
    assert_eq!(
        client_derive_reqwest_fn_body_request_handler::body_request_handler(
            &client,
            route,
            Bytes::from("Hello World."),
        ).await.unwrap().status(),
        StatusCode::OK
    );
}

#[reqwest_fn("get")]
async fn type_header_handler_with_tuple(
    TypedHeader(jwt): TypedHeader<Jwt>,
    state:State<ArbState>,
) -> StatusCode {
    println!("{}",jwt.0);
    StatusCode::OK
}

#[tokio::test]
async fn test_type_header_handler_with_tuple() {
    let router = Router::with_state(ArbState)
        .route("/hello",
               get(type_header_handler_with_tuple));
    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();

    let route = format!("http://{}{}",addr,"/hello");

    eprintln!("{}",route);
    assert_eq!(
        client_derive_reqwest_fn_type_header_handler_with_tuple
        ::type_header_handler_with_tuple
            (
            &client,
            route,
            Jwt("Hello world.".into())
        ).await.unwrap().status(),
        StatusCode::OK
    );
}

#[reqwest_fn("get")]
async fn type_header_handler_with_tuple_struct(
    Json(Email{email}): Json<Email>,
) -> StatusCode {
    StatusCode::OK
}

#[tokio::test]
async fn test_type_header_handler_with_tuple_struct() {
    let router = Router::new()
        .route("/hello",
               get(type_header_handler_with_tuple_struct));
    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();

    let route = format!("http://{}{}",addr,"/hello");

    eprintln!("{}",route);
    assert_eq!(
        client_derive_reqwest_fn_type_header_handler_with_tuple_struct
        ::type_header_handler_with_tuple_struct
            (
                &client,
                route,
                &Email{email:"...".into()}
            ).await.unwrap().status(),
        StatusCode::OK
    );
}
#[derive(Serialize,Deserialize,Clone,Debug,PartialEq)]
pub struct User{
    email:String,
    pass:String,
}
#[reqwest_fn("put")]
async fn type_header_handler_with_struct(
    Json(User{email,..}): Json<User>,
) -> StatusCode {
    StatusCode::OK
}

#[tokio::test]
async fn test_type_header_handler_with_struct() {
    let router = Router::new()
        .route("/hello",
               put(type_header_handler_with_struct));
    let listener = TcpListener::bind("0.0.0.0:0"
        .parse::<SocketAddr>()
        .unwrap()
    ).unwrap();

    let addr = listener.local_addr().unwrap().to_string();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(router.into_make_service())
            .await
            .unwrap();
    });
    let client = Client::new();

    let route = format!("http://{}{}",addr,"/hello");

    eprintln!("{}",route);
    assert_eq!(
        client_derive_reqwest_fn_type_header_handler_with_struct
        ::type_header_handler_with_struct
            (
                &client,
                route,
                &User{email:"...".into(),pass:"PASSWORD".into()}
            ).await.unwrap().status(),
        StatusCode::OK
    );
}

