use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignAnalytics {
    pub campaign_id: String,
    pub campaign_name: String,
    pub total_sent: i64,
    pub total_opened: i64,
    pub total_clicked: i64,
    pub open_rate: f64,
    pub click_rate: f64,
    pub departments: Vec<DepartmentStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentStats {
    pub department: String,
    pub employee_count: i64,
    pub emails_sent: i64,
    pub emails_opened: i64,
    pub links_clicked: i64,
    pub open_rate: f64,
    pub click_rate: f64,
}
