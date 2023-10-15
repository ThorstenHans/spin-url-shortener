use anyhow::Result;
use rand::Rng;
use serde::Deserialize;
use serde_json::json;
use spin_sdk::{
    http::{Params, Request, Response, Router},
    http_component,
    key_value::Store,
};
use std::collections::HashMap;

const SHORT_CODE_LENGTH: usize = 5;
const PATH_TO_FRONTEND_APP: &str = "app/";
#[http_component]
fn handle_api(req: Request) -> Result<Response> {
    let mut router = Router::default();
    router.get("/links", get_all_links);
    router.post("/links", create_link);
    router.get("/:short", redirect_to_target);
    router.get("/", redirect_to_app);
    router.any("*", not_found);
    router.handle(req)
}

#[derive(Deserialize)]
pub struct CreateLinkModel {
    pub url: String,
}

fn redirect_to_app(_: Request, _: Params) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::MOVED_PERMANENTLY)
        .header(http::header::LOCATION, PATH_TO_FRONTEND_APP)
        .body(None)?)
}

fn redirect_to_target(_: Request, params: Params) -> Result<Response> {
    let Some(short_code) = params.get("short") else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };

    let store = Store::open_default()?;
    let Ok(target) = store.get(short_code) else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::NOT_FOUND)
            .body(None)?);
    };
    let target = String::from_utf8(target)?;

    Ok(http::Response::builder()
        .status(http::StatusCode::MOVED_PERMANENTLY)
        .header("Location", target)
        .body(None)?)
}

fn create_link(req: Request, _: Params) -> Result<Response> {
    let Some(body) = req.body().clone() else {
        return Ok(http::Response::builder()
            .status(http::StatusCode::BAD_REQUEST)
            .body(None)?);
    };
    let model: CreateLinkModel = serde_json::from_slice(&body)?;
    let short = create_short_code();
    let store = Store::open_default()?;
    store.set(&short, &model.url)?;

    let payload = json!({
        "short": short,
        "url": model.url,
    })
    .to_string();

    Ok(http::Response::builder()
        .status(http::StatusCode::CREATED)
        .body(Some(payload.into()))?)
}

fn get_all_links(_: Request, _: Params) -> Result<Response> {
    let store = Store::open_default()?;
    let mut links: HashMap<String, String> = HashMap::new();
    let keys = store.get_keys()?;
    for key in keys {
        let value = store.get(&key)?;
        match String::from_utf8(value) {
            Ok(target) => {
                links.insert(key, target);
            }
            Err(_) => continue,
        }
    }
    let payload = serde_json::to_string(&links)?;
    Ok(http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Some(payload.into()))?)
}

fn not_found(_: Request, _: Params) -> Result<Response> {
    Ok(http::Response::builder()
        .status(http::StatusCode::NOT_FOUND)
        .body(None)?)
}

fn create_short_code() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(SHORT_CODE_LENGTH)
        .map(char::from)
        .collect()
}
