use axum::extract::{FromRef, Path, State};
use axum::{Json, Router};
use axum::routing::{delete, get, post};
use crate::model::{ModelController, Ticket, TickerForCreate};
use crate::Result;
pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets",post(create_ticket).get(list_tickets))
        .route("/tickets/:id",delete(delete_tickets))
        .with_state(mc)
}

// region: --- REST Handlers

async fn create_ticket(
    State(mc):State<ModelController>,
    Json(ticket_fc): Json<TickerForCreate>
) -> Result<Json<Ticket>> {
    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc):State<ModelController>
) -> Result<Json<Vec<Ticket>>> {
    let tickets = mc.list_tickets().await?;

    Ok(Json(tickets))
}

async fn delete_tickets(
    State(mc): State<ModelController>,
    Path(id): Path<u64>
) -> Result<Json<Ticket>> {
    let result = mc.delete_tickets(id).await?;

    Ok(Json(result))
}

// endregion: --- REST Handlers