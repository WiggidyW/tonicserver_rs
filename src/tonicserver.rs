use crate::{env, error::Error};
use std::fmt::Display;
use tonic::transport::Server as TonicServer;

struct TonicServerBuilder(TonicServer);

impl TonicServerBuilder {
    fn with_opt<T>(self, method: fn(TonicServer, T) -> TonicServer, opt: Option<T>) -> Self {
        match opt {
            Some(opt) => Self(method(self.0, opt)),
            None => self,
        }
    }
    fn with_optional_opt<T>(
        self,
        method: fn(TonicServer, Option<T>) -> TonicServer,
        opt: Option<T>,
    ) -> Self {
        match opt {
            Some(opt) => Self(method(self.0, Some(opt))),
            None => self,
        }
    }
}

pub fn new_tonic_server(namespace: impl Display) -> Result<tonic::transport::Server, Error> {
    Ok(TonicServerBuilder(TonicServer::builder())
        // .with_opt!(concurrency_limit_per_connection)
        .with_optional_opt(TonicServer::tcp_keepalive, env::tcp_keepalive(&namespace)?)
        .with_opt(TonicServer::accept_http1, env::accept_http1(&namespace)?)
        .with_opt(TonicServer::tcp_nodelay, env::tcp_nodelay(&namespace)?)
        .with_opt(TonicServer::timeout, env::timeout(&namespace)?)
        .with_opt(
            TonicServer::concurrency_limit_per_connection,
            env::concurrency_limit_per_connection(&namespace)?,
        )
        .with_optional_opt(
            TonicServer::initial_connection_window_size,
            env::initial_connection_window_size(&namespace)?,
        )
        .with_optional_opt(
            TonicServer::initial_stream_window_size,
            env::initial_stream_window_size(&namespace)?,
        )
        .with_optional_opt(
            TonicServer::http2_keepalive_interval,
            env::http2_keepalive_interval(&namespace)?,
        )
        .with_optional_opt(
            TonicServer::http2_keepalive_timeout,
            env::http2_keepalive_timeout(&namespace)?,
        )
        .with_optional_opt(
            TonicServer::max_concurrent_streams,
            env::max_concurrent_streams(&namespace)?,
        )
        .with_optional_opt(
            TonicServer::http2_adaptive_window,
            env::http2_adaptive_window(&namespace)?,
        )
        .with_optional_opt(
            TonicServer::max_frame_size,
            env::max_frame_size(&namespace)?,
        )
        .0)
}
