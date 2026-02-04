use chrono::Timelike;
use store::Store;


pub struct CheckResult {
    pub website_id : String,
    pub is_up: bool,
    pub response_time_ms: u64,
    pub error_msg: Option<String>,
    pub previous_status: String,
}

pub async fn process_check_result(store: &Store, result: CheckResult) -> Result<(), Box<dyn std::error::Error>> {
    let current_status = if result.is_up { "UP" } else { "DOWN" };

    //ASE 1: EMERGENCY (STATUS CHANGE)
    if current_status != result.previous_status {
        println!("Status Changed: {} -> {}", result.previous_status, current_status);
        store.handle_incident_log(&result.website_id, current_status, result.error_msg).await?;
        store.update_status_change(&result.website_id, current_status).await?;
        store.update_reliability(&result.website_id, result.is_up, 60).await?;
    } 
    
    // ROUTINE CHECK (BULK UPDATE)
    else {
        let is_sync_time = chrono::Utc::now().minute() % 5 == 0;

        if is_sync_time {
            println!("5-Minute Sync: Bulk Updating All Tables...");
            store.update_last_checked_routine(&result.website_id, current_status).await?;
            store.update_reliability(&result.website_id, result.is_up, 300).await?;
            
        } else {
            println!("Skipping DB Write (Waiting for 5-min mark)");
        }
    }

    Ok(())
}