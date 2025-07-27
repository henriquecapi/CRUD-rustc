use tide::http::{Request, Response, Method, Url};
use serde_json::json;
use async_std::task;
use crud_henriquecapi::{models::DataEntry, setup_app};

#[test]
fn test_crud_flow() {
    task::block_on(async {
        let app = setup_app();

        // 1. Create
        let mut req = Request::new(Method::Post, Url::parse("http://localhost/data").unwrap());
        let payload = json!({
            "data1": ["test"],
            "data2": [1, 2, 3]
        });
        req.set_body(payload.to_string());
        req.insert_header("Content-Type", "application/json");

        let mut res: Response = app.respond(req).await.unwrap();
        assert_eq!(res.status(), 200);
        let body: serde_json::Value = res.body_json().await.unwrap();
        let new_id = body["id"].as_u64().unwrap();
        assert_eq!(new_id, 1);

        // 2. Read One
        let req = Request::new(Method::Get, Url::parse(&format!("http://localhost/data/{}", new_id)).unwrap());
        let mut res: Response = app.respond(req).await.unwrap();
        assert_eq!(res.status(), 200);
        let entry: DataEntry = res.body_json().await.unwrap();
        assert_eq!(entry.data1, vec!["test"]);
        assert_eq!(entry.data2, vec![1, 2, 3]);

        // 3. Read All
        let req = Request::new(Method::Get, Url::parse("http://localhost/data").unwrap());
        let mut res: Response = app.respond(req).await.unwrap();
        assert_eq!(res.status(), 200);
        let entries: Vec<DataEntry> = res.body_json().await.unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].data1, vec!["test"]);

        // 4. Update
        let mut req = Request::new(Method::Put, Url::parse(&format!("http://localhost/data/{}", new_id)).unwrap());
        let updated_payload = json!({
            "data1": ["updated"],
            "data2": [4, 5, 6]
        });
        req.set_body(updated_payload.to_string());
        req.insert_header("Content-Type", "application/json");
        let res: Response = app.respond(req).await.unwrap();
        assert_eq!(res.status(), 200);

        // Verify Update
        let req = Request::new(Method::Get, Url::parse(&format!("http://localhost/data/{}", new_id)).unwrap());
        let mut res: Response = app.respond(req).await.unwrap();
        let entry: DataEntry = res.body_json().await.unwrap();
        assert_eq!(entry.data1, vec!["updated"]);
        assert_eq!(entry.data2, vec![4, 5, 6]);

        // 5. Delete
        let req = Request::new(Method::Delete, Url::parse(&format!("http://localhost/data/{}", new_id)).unwrap());
        let res: Response = app.respond(req).await.unwrap();
        assert_eq!(res.status(), 200);

        // Verify Delete
        let req = Request::new(Method::Get, Url::parse(&format!("http://localhost/data/{}", new_id)).unwrap());
        let res: Response = app.respond(req).await.unwrap();
        assert_eq!(res.status(), 404);
    });
}
