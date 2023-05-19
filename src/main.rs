use axum::{
    extract::ConnectInfo,
    http::{HeaderMap, StatusCode},
    routing::get,
    Router,
};
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 4004));
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn root<'a>(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> (StatusCode, String) {
    (
        StatusCode::OK,
        headers.get("X-Forwarded-For").map_or_else(
            || addr.ip().to_string(),
            |v| {
                v.to_str()
                    .map(|v| IpAddr::from_str(v).map_or_else(|_| "".to_string(), |x| x.to_string()))
                    .unwrap()
            },
        ),
    )
}
