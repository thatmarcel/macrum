use std::convert::Infallible;
use std::net::{Ipv4Addr, SocketAddrV4};
use axum::Router;
use axum::routing::MethodRouter;

pub trait RouterMacrumExt: Send {
    fn include(self, route_include_fn: impl FnOnce() -> (String, MethodRouter<(), Infallible>)) -> Router<()>;

    fn serve_on_port_from_env_or(self, port: u16) -> impl Future<Output = Result<(), std::io::Error>> + Send;
}

impl RouterMacrumExt for Router {
    fn include(self, route_include_fn: impl FnOnce() -> (String, MethodRouter<(), Infallible>)) -> Router<()> {
        let (path, method_router) = route_include_fn();
        self.route(&*path, method_router)
    }

    async fn serve_on_port_from_env_or(self, default_port: u16) -> Result<(), std::io::Error> {
        let listener = tokio::net::TcpListener::bind(
            SocketAddrV4::new(
                Ipv4Addr::UNSPECIFIED,
                std::env::var("PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(default_port)
            )
        ).await?;

        println!("Listening on port {}...", listener.local_addr()?.port());

        axum::serve(listener, self).await
    }
}