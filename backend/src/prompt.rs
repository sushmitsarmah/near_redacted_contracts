use std::env;
use reqwest::header::HeaderMap;

#[axum::debug_handler]
async fn prompt(
    State(state): State<AppState>,
    Json(prompt): Json<Prompt>,
) -> Result<impl IntoResponse, ApiError> {
    let data = state.researcher.prepare_data(&prompt.q).await?;
    let resarcher_result = state.researcher.prompt(&prompt.q, data).await?;
    let writer_result = state.writer.prompt(&prompt.q, res).await?;

    Ok(writer_result)
}