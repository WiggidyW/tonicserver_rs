use crate::error::Error;

use std::{fmt::Display, net::SocketAddr, time::Duration};

use env_util;

const CONCURRENCY_LIMIT_PER_CONNECTION: &str = "TS_CONCURRENCY_LIMIT_PER_CONNECTION";
const TIMEOUT: &str = "TS_TIMEOUT";
const INITIAL_STREAM_WINDOW_SIZE: &str = "TS_INITIAL_STREAM_WINDOW_SIZE";
const INITIAL_CONNECTION_WINDOW_SIZE: &str = "TS_INITIAL_CONNECTION_WINDOW_SIZE";
const MAX_CONCURRENT_STREAMS: &str = "TS_MAX_CONCURRENT_STREAMS";
const HTTP2_KEEPALIVE_INTERVAL: &str = "TS_HTTP2_KEEPALIVE_INTERVAL";
const HTTP2_KEEPALIVE_TIMEOUT: &str = "TS_HTTP2_KEEPALIVE_TIMEOUT";
const HTTP2_ADAPTIVE_WINDOW: &str = "TS_HTTP2_ADAPTIVE_WINDOW";
const TCP_KEEPALIVE: &str = "TS_TCP_KEEPALIVE";
const TCP_NODELAY: &str = "TS_TCP_NODELAY";
const MAX_FRAME_SIZE: &str = "TS_MAX_FRAME_SIZE";
const ACCEPT_HTTP1: &str = "TS_ACCEPT_HTTP1";

const SERVICE_ADDRESS: &str = "TS_SERVICE_ADDRESS";

pub fn concurrency_limit_per_connection(namespace: impl Display) -> Result<Option<usize>, Error> {
    let key = format!("{namespace}_{CONCURRENCY_LIMIT_PER_CONNECTION}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(v.then_try_fromstr_into()?.into_inner()),
        None => None,
    })
}

pub fn timeout(namespace: impl Display) -> Result<Option<Duration>, Error> {
    let key = format!("{namespace}_{TIMEOUT}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(
            v.then_try_fromstr_into()?
                .then_fn_into(Duration::from_secs)
                .into_inner(),
        ),
        None => None,
    })
}

pub fn initial_stream_window_size(namespace: impl Display) -> Result<Option<u32>, Error> {
    let key = format!("{namespace}_{INITIAL_STREAM_WINDOW_SIZE}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(v.then_try_fromstr_into()?.into_inner()),
        None => None,
    })
}

pub fn initial_connection_window_size(namespace: impl Display) -> Result<Option<u32>, Error> {
    let key = format!("{namespace}_{INITIAL_CONNECTION_WINDOW_SIZE}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(v.then_try_fromstr_into()?.into_inner()),
        None => None,
    })
}

pub fn max_concurrent_streams(namespace: impl Display) -> Result<Option<u32>, Error> {
    let key = format!("{namespace}_{MAX_CONCURRENT_STREAMS}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(v.then_try_fromstr_into()?.into_inner()),
        None => None,
    })
}

pub fn http2_keepalive_interval(namespace: impl Display) -> Result<Option<Duration>, Error> {
    let key = format!("{namespace}_{HTTP2_KEEPALIVE_INTERVAL}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(
            v.then_try_fromstr_into()?
                .then_fn_into(Duration::from_secs)
                .into_inner(),
        ),
        None => None,
    })
}

pub fn http2_keepalive_timeout(namespace: impl Display) -> Result<Option<Duration>, Error> {
    let key = format!("{namespace}_{HTTP2_KEEPALIVE_TIMEOUT}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(
            v.then_try_fromstr_into()?
                .then_fn_into(Duration::from_secs)
                .into_inner(),
        ),
        None => None,
    })
}

pub fn http2_adaptive_window(namespace: impl Display) -> Result<Option<bool>, Error> {
    let key = format!("{namespace}_{HTTP2_ADAPTIVE_WINDOW}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(v.then_try_fromstr_into()?.into_inner()),
        None => None,
    })
}

pub fn tcp_keepalive(namespace: impl Display) -> Result<Option<Duration>, Error> {
    let key = format!("{namespace}_{TCP_KEEPALIVE}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(
            v.then_try_fromstr_into()?
                .then_fn_into(Duration::from_secs)
                .into_inner(),
        ),
        None => None,
    })
}

pub fn tcp_nodelay(namespace: impl Display) -> Result<Option<bool>, Error> {
    let key = format!("{namespace}_{TCP_NODELAY}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(v.then_try_fromstr_into()?.into_inner()),
        None => None,
    })
}

pub fn max_frame_size(namespace: impl Display) -> Result<Option<u32>, Error> {
    let key = format!("{namespace}_{MAX_FRAME_SIZE}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(v.then_try_fromstr_into()?.into_inner()),
        None => None,
    })
}

pub fn accept_http1(namespace: impl Display) -> Result<Option<bool>, Error> {
    let key = format!("{namespace}_{ACCEPT_HTTP1}");
    Ok(match env_util::get(&key).optional_checked()? {
        Some(v) => Some(v.then_try_fromstr_into()?.into_inner()),
        None => None,
    })
}

pub fn service_address(namespace: impl Display) -> Result<SocketAddr, Error> {
    let key = format!("{namespace}_{SERVICE_ADDRESS}");
    Ok(env_util::get(&key)
        .required_checked()?
        .then_try_fromstr_into()?
        .into_inner())
}
