use crate::models::DataEntry;
use crate::state::AppState;
use tide::{Request, Response, Body, StatusCode};
use serde_json::json;

pub async fn update_data(mut req: Request<AppState>) -> tide::Result {
    let id: u32 = req.param("id")?.parse()?;
    let entry: DataEntry = req.body_json().await?;

    let state = req.state();
    let mut map = state.lock().unwrap();

    if map.contains_key(&id) {
        map.insert(id, entry);
        Ok(Response::new(StatusCode::Ok))
    } else {
        let mut res = Response::new(StatusCode::NotFound);
        res.set_body(Body::from_json(&json!({ "error": "not found" }))?);
        Ok(res)
    }
}
