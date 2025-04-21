use axum::{ routing::{ get, patch, post, put, delete }, Error, Router }; // Adicionado put, delete
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    modules::{
        arenas::{
            arenas_controller::list_free_times_controller,
            arenas_model::{ Estabelecimento, RegisterQuadraInput },
            estabelecimento::controller::{
                delete_estabelecimento_controller,
                get_all_estabelecimentos_handler,
                get_estabelecimento_controller,
                register_estabelecimento_controller,
                update_estabelecimento_controller,
            },
            horario::controller::{
                delete_all_horarios_controller, // Adicionado delete_all_horarios_controller
                delete_single_horario_controller, // Adicionado delete_single_horario_controller
                find_horarios_by_quadra_id_controller,
                update_horarios_controller, // Adicionado update_horarios_controller
                update_single_horario_controller, // Adicionado update_single_horario_controller
            },
            local::controller::{
                delete_local_controller,
                get_locais_controller,
                update_locais_controller,
            },
            quadra::controller::{
                delete_quadra_controller,
                find_quadra_by_id_controller,
                register_quadras_controller,
                update_quadras_controller,
            },
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
        .route("/quadras", post(register_quadras_controller::<RegisterQuadraInput>))
        .route(
            "/quadras/:id",
            patch(update_quadras_controller)
                .get(find_quadra_by_id_controller)
                .delete(delete_quadra_controller)
        )
        .route(
            "/quadras/:id/horarios",
            get(find_horarios_by_quadra_id_controller) // GET all horarios for quadra
                .put(update_horarios_controller) // PUT (replace) all horarios for quadra
                .delete(delete_all_horarios_controller) // DELETE all horarios for quadra
        )
        .route(
            "/quadras/:id/horarios/:dia_semana",
            patch(update_single_horario_controller) // PATCH specific day
                .delete(delete_single_horario_controller) // DELETE specific day
        )
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
