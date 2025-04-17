use axum::{ routing::{ get, patch, post }, Error, Router };
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    modules::{
        arenas::{
            arenas_controller::{
                delete_estabelecimento_controller,
                delete_local_controller,
                get_all_estabelecimentos_handler,
                get_estabelecimento_controller,
                get_locais_controller,
                list_free_times_controller,
                register_estabelecimento_controller,
                register_quadras_controller,
                update_estabelecimento_controller,
                update_locais_controller,
            },
            arenas_model::{ Estabelecimento, RegisterQuadraInput },
        },
        auth::auth_controller::{ login_controller, register_user_controller },
        rent::rent_controller::{ register_reserva_controller, update_reserva_status_controller },
    },
    AppState,
};

pub fn create_router(app_state: Arc<Mutex<AppState>>) -> Result<Router, Error> {
    let auth_routes: Router<Arc<Mutex<AppState>>> = Router::new()
        .route("/auth/login", post(login_controller))
        .route("/auth/register", post(register_user_controller));

    let arenas_routes = Router::new()
        .route("/organization", post(register_estabelecimento_controller::<Estabelecimento>))
        .route("/estabelecimentos", get(get_all_estabelecimentos_handler)) // Adicionando a nova rota
        .route(
            "/estabelecimentos/:id",
            get(get_estabelecimento_controller)
                .delete(delete_estabelecimento_controller)
                .patch(update_estabelecimento_controller)
        )
        .route("/venue", post(register_quadras_controller::<RegisterQuadraInput>))
        .route(
            "/locais/:id",
            get(get_locais_controller)
                .patch(update_locais_controller)
                .delete(delete_local_controller)
        );

    let rent_routes = Router::new()
        .route("/", post(register_reserva_controller))
        .route("/:id", patch(update_reserva_status_controller))
        .route("/available-time/:date/:quadra_id", get(list_free_times_controller));

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

    Ok(
        Router::new()
            .nest("/api", auth_routes)
            .nest("/api/arenas", arenas_routes)
            .nest("/api/rent", rent_routes)
            .with_state(app_state)
    )
}
