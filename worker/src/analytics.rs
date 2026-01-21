use store::Store;

pub struct CheckResult {
    pub website_id : String,
    pub is_up: bool,
    pub response_time_ms: u64,
    pub error_msg: Option<String>,
}

pub async fn process_check_result(store: &Store, result: CheckResult) -> Result<(), Box<dyn std::error::Error>> {
    let current_status = if result.is_up { "UP" } else { "DOWN" };

    store.update_last_checked(&result.website_id, current_status).await?;

    store.update_website_stats(&result.website_id, current_status).await?;

    if !result.is_up {
        store.handle_incident_log(&result.website_id, "DOWN", result.error_msg).await?;
    } else {
        store.handle_incident_log(&result.website_id, "UP", None).await?;
    }
    Ok(())
}