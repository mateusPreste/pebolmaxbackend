use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

use crate::{
    controller::note_controller::{
        create_note_controller, delete_note_controller, edit_note_controller, get_note_controller,
        healthchecker_controller, note_list_controller,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(healthchecker_controller))
        .route("/api/notes", get(note_list_controller))
        .route("/api/notes/", post(create_note_controller))
        .route(
            "/api/notes/:id",
            get(get_note_controller)
                .patch(edit_note_controller)
                .delete(delete_note_controller),
        )
        .with_state(app_state)
}
