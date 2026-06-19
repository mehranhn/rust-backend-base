use axum::{Router, http::StatusCode};
use memory_serve::{MemoryServe, load_assets};

pub fn web_ui_router() -> Router {
    MemoryServe::new(load_assets!("./web_ui"))
        .index_file(Some("/index.html"))
        .fallback(Some("/index.html"))
        .fallback_status(StatusCode::OK)
        .enable_gzip(true)
        .enable_brotli(true)
        .html_cache_control(memory_serve::CacheControl::Medium)
        .cache_control(memory_serve::CacheControl::Medium)
        .into_router()
}
