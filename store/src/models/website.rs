use crate::store::Store;
use uuid::Uuid;

pub struct Output {
    pub id: String,
    pub url: String,
    pub check_interval: Option<i32>,
}

impl Store {
    pub async fn create_websites(&self, url: String, user_id: String, name: String, check_interval: i32) -> Result<Output, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let res = sqlx::query!(
            "INSERT INTO websites (id,user_id,name,url,check_interval) VALUES ($1,$2,$3,$4,$5) RETURNING id,url,check_interval",
            id, user_id, name, url, check_interval
        )
        .fetch_one(&self.conn)
        .await?;
    
        // Initialize website_stats entry
        sqlx::query!(
            "INSERT INTO website_stats (website_id, current_status, last_status_change, total_uptime_seconds, total_downtime_seconds) 
             VALUES ($1, 'UP'::text::website_status, NOW(), 0, 0)",
            id
        )
        .execute(&self.conn)
        .await?;

        Ok(Output {
            id: res.id,
            url: res.url,
            check_interval: res.check_interval,
        })
    }

    pub async fn update_last_checked(&self, website_id: &str, status: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE websites SET last_checked_at = NOW(), status = $1::text::website_status WHERE id = $2",
            status, website_id
        )
        .execute(&self.conn)
        .await?;
        Ok(())
    }

    pub async fn update_website_stats(&self, website_id: &str, current_check_status: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE website_stats 
            SET 
                total_uptime_seconds = total_uptime_seconds + 
                    CASE WHEN current_status = 'UP' THEN EXTRACT(EPOCH FROM (NOW() - last_status_change))::BIGINT ELSE 0 END,
                total_downtime_seconds = total_downtime_seconds + 
                    CASE WHEN current_status = 'DOWN' THEN EXTRACT(EPOCH FROM (NOW() - last_status_change))::BIGINT ELSE 0 END,
                last_status_change = NOW(),
                current_status = $2::text::website_status
            WHERE website_id = $1
            "#,
            website_id,
            current_check_status
        )
        .execute(&self.conn)
        .await?;

        Ok(())
    }

    pub async fn handle_incident_log(
        &self,
        website_id: &str,
        new_status: &str,
        error_msg: Option<String>,
    ) -> Result<(), sqlx::Error> {
        if new_status == "DOWN" {
            let active_incident = sqlx::query!(
                "SELECT id FROM incidents WHERE website_id = $1 AND end_time IS NULL", 
                website_id
            )
            .fetch_optional(&self.conn)
            .await?;
            if active_incident.is_none() {
                let incident_id = Uuid::new_v4().to_string();
                sqlx::query!(
                    "INSERT INTO incidents (id, website_id, error_reason, start_time) VALUES ($1, $2, $3, NOW())",
                    incident_id,
                    website_id,
                    error_msg
                )
                .execute(&self.conn)
                .await?;
            }
        } else {
            sqlx::query!(
                "UPDATE incidents SET end_time = NOW() 
                 WHERE website_id = $1 AND end_time IS NULL",
                website_id
            )
            .execute(&self.conn)
            .await?;
        }

        Ok(())
    }
}