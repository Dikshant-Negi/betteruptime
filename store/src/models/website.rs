use crate::store::Store;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct DailyReliabilityStat {
    pub date: NaiveDate,
    pub down_seconds: i64,
    pub up_seconds: i64,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Website {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub url: String,
    pub check_interval: i32,
    pub status: Option<String>, 
    pub last_checked_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>, 
}

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

    pub async fn get_websites(&self, user_id: &str) -> Result<Vec<Website>, sqlx::Error> {
        sqlx::query_as::<_, Website>(
            r#"
            SELECT 
                id, user_id, name, url, check_interval, 
                status::text, -- Cast ENUM to TEXT
                last_checked_at, created_at 
            FROM websites 
            WHERE user_id = $1 
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.conn)
        .await
        
    }
    pub async fn update_last_checked_routine(&self, website_id: &str, new_status: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE websites SET last_checked_at = NOW(), status = $1::text::website_status WHERE id = $2",
            new_status, website_id
        )
        .execute(&self.conn)
        .await?;
        Ok(())
    }

    pub async fn update_reliability(&self, website_id: &str, is_up: bool, seconds_to_add: i64) -> Result<(), sqlx::Error> {
        let today = chrono::Local::now().date_naive();
        
        let (up_inc, down_inc) = if is_up {
            (seconds_to_add, 0) 
        } else {
            (0, seconds_to_add) 
        };

        sqlx::query!(
            r#"
            INSERT INTO daily_reliability (website_id, date, up_seconds, down_seconds)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (website_id, date) 
            DO UPDATE SET 
                up_seconds = daily_reliability.up_seconds + $3,
                down_seconds = daily_reliability.down_seconds + $4
            "#,
            website_id,
            today,
            up_inc,
            down_inc
        )
        .execute(&self.conn)
        .await?;

        sqlx::query!(
        r#"
        UPDATE website_stats
        SET 
            total_uptime_seconds = total_uptime_seconds + $2,
            total_downtime_seconds = total_downtime_seconds + $3
        WHERE website_id = $1
        "#,
        website_id, up_inc, down_inc
    )
    .execute(&self.conn).await?;
        Ok(())
    }

    pub async fn update_status_change(&self, website_id: &str, new_status: &str) -> Result<(), sqlx::Error> {
        
        sqlx::query!(
            "UPDATE websites SET last_checked_at = NOW(), status = $1::text::website_status WHERE id = $2",
            new_status, website_id
        )
        .execute(&self.conn)
        .await?;
        
        sqlx::query!(
            r#"
            UPDATE website_stats SET
                total_uptime_seconds = total_uptime_seconds + CASE WHEN current_status ='UP' THEN EXTRACT(EPOCH FROM (NOW() - last_status_change))::BIGINT ELSE 0 END,
                total_downtime_seconds = total_downtime_seconds + CASE WHEN current_status ='DOWN' THEN EXTRACT(EPOCH FROM (NOW() - last_status_change))::BIGINT ELSE 0 END,

                last_status_change = NOW(),
                current_status = $2::text::website_status
            WHERE website_id = $1
            "#,
            website_id, new_status
        )
        .execute(&self.conn)
        .await?;
        Ok(())
    }

    pub async fn get_daily_reliability(&self, website_id: &str) -> Result<Vec<DailyReliabilityStat>, sqlx::Error> {
        sqlx::query_as::<_, DailyReliabilityStat>(
            r#"
            SELECT 
                date, 
                up_seconds, 
                down_seconds
            FROM daily_reliability
            WHERE website_id = $1
            ORDER BY date DESC
            LIMIT 30
            "#
        )
        .bind(website_id)
        .fetch_all(&self.conn)
        .await
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