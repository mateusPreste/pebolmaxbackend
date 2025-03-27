use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

use crate::{
    controller::note_controller::{
        create_note_controller, delete_note_controller, edit_note_controller, get_note_controller,
        healthchecker_controller, note_list_controller,
    },
    middleware::auth_middleware,
    modules::auth::auth_controller::{login_controller, register_user_controller},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let auth_routes = Router::new()
        .route("/login", post(login_controller))
        .route("/register", post(register_user_controller));

    let protected_routes = Router::new()
        .route("/notes", get(note_list_controller))
        .route("/notes/", post(create_note_controller))
        .route(
            "/notes/:id",
            get(get_note_controller)
                .patch(edit_note_controller)
                .delete(delete_note_controller),
        )
        .route("/user/:id/notes", get(note_list_controller))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    Router::new()
        .route("/healthchecker", get(healthchecker_controller))
        .nest("/api", auth_routes)
        .nest("/api", protected_routes)
        .with_state(app_state)
}
