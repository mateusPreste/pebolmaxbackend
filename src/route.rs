use axum::{
    middleware,
    routing::{delete, get, patch, post, MethodRouter},
    Error, Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    controller::note_controller::{
        create_note_controller, delete_note_controller, edit_note_controller, get_note_controller,
        healthchecker_controller, note_list_controller,
    },
    middleware::auth_middleware,
    modules::{
        arenas::{
            arenas_controller::{
                list_free_times_controller, register_estabelecimento_controller,
                register_quadras_controller,
            },
            arenas_model::{Estabelecimento, RegisterQuadraInput},
        },
        auth::auth_controller::{login_controller, register_user_controller},
        payments::payment_controller::{create_payment_controller, update_payment_status_controller},
        rent::rent_controller::{register_reserva_controller, update_reserva_status_controller, get_reserva_details_controller},
    },
    AppState,
};

pub fn create_router(app_state: Arc<Mutex<AppState>>) -> Result<Router, Error> {
    let auth_routes: Router<Arc<Mutex<AppState>>> = Router::new()
        .route("/auth/login", post(login_controller))
        .route("/auth/register", post(register_user_controller));

    let arenas_routes = Router::new()
        .route(
            "/organization",
            post(register_estabelecimento_controller::<Estabelecimento>),
        )
        .route(
            "/venue",
            post(register_quadras_controller::<RegisterQuadraInput>),
        );

    let rent_routes = Router::new()
        .route("/", post(register_reserva_controller))
        .route("/:id", patch(update_reserva_status_controller))
        .route("/:id", get(get_reserva_details_controller))
        .route(
            "/available-time/:date/:quadra_id",
            get(list_free_times_controller),
        );

    let payment_routes = Router::new()
        .route("/", post(create_payment_controller))
        .route("/:id/status", patch(update_payment_status_controller));

    /* let protected_routes = Router::new()
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
    )); */

    Ok(Router::new()
        .nest("/api", auth_routes)
        .nest("/api/arenas", arenas_routes)
        .nest("/api/rent", rent_routes)
        .nest("/api/payments", payment_routes)
        .with_state(app_state))
}
