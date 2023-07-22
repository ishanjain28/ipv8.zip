use axum::{extract::ConnectInfo, http::HeaderMap, routing::get, Router};
use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4004));
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn root<'a>(ConnectInfo(addr): ConnectInfo<SocketAddr>, headers: HeaderMap) -> String {
    headers.get("X-Forwarded-For").map_or_else(
        || addr.ip().to_string(),
        |v| {
            v.to_str()
                .map(|v| {
                    v.split_once(',').map_or_else(
                        || IpAddr::from_str(v).map_or_else(|_| "".to_string(), |x| x.to_string()),
                        |(ip, _)| {
                            IpAddr::from_str(ip).map_or_else(|_| "".to_string(), |x| x.to_string())
                        },
                    )
                })
                .unwrap()
        },
    )
}
