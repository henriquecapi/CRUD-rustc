pub mod handlers;
pub mod models;
pub mod state;

use tide::Server;
use state::{new_state, AppState};
use handlers::{
    create::create_data,
    read::{read_all_data, read_data},
    update::update_data,
    delete::delete_data,
};

pub fn setup_app() -> Server<AppState> {
    let state = new_state();
    let mut app = tide::with_state(state);

    app.at("/data").post(create_data);
    app.at("/data").get(read_all_data);
    app.at("/data/:id").get(read_data);
    app.at("/data/:id").put(update_data);
    app.at("/data/:id").delete(delete_data);

    app
}