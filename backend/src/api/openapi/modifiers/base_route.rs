use indexmap::IndexMap;
use utoipa::{Modify, openapi::PathItem};

pub struct BaseRoute;

impl Modify for BaseRoute {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let mut new_paths: IndexMap<String, PathItem> = IndexMap::new();
        for (path, item) in openapi.paths.paths.iter() {
            new_paths.insert(format!("/api{}", path), item.clone());
        }
        openapi.paths.paths = new_paths;
    }
}
