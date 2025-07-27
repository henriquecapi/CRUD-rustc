use crate::state::AppState;
use tide::{Request, Body, Response, StatusCode};
use serde_json::json;

// Handler para ler todos os dados
pub async fn read_all_data(req: Request<AppState>) -> tide::Result {
    let state = req.state();
    let map = state.lock().unwrap();
    let data: Vec<_> = map.values().cloned().collect();
    Ok(Body::from_json(&data)?.into())
}

// Handler para ler um dado específico
pub async fn read_data(req: Request<AppState>) -> tide::Result {
    let id: u32 = req.param("id")?.parse()?;
    let state = req.state();
    let map = state.lock().unwrap();

    if let Some(entry) = map.get(&id) {
        Ok(Body::from_json(entry)?.into())
    } else {
        let mut res = Response::new(StatusCode::NotFound);
        res.set_body(Body::from_json(&json!({ "error": "not found" }))?);
        Ok(res)
    }
}
