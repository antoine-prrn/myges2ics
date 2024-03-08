use reqwest::header;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Project {
    pub author: String,
    pub name: String,
    pub course_name: String,
    pub steps: Option<Vec<Step>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Step {
    pub psp_type: String,
    pub psp_desc: String,
    pub psp_limit_date: Option<u128>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectStep {
    pub course_name: String,
    pub pro_name: String,
    pub psp_desc: String,
    pub psp_type: String,
    pub psp_limit_date: Option<u128>,
}

#[derive(Deserialize, Debug)]
pub struct NextProjectStepsWebResult {
    result: Vec<ProjectStep>,
}

pub async fn get_next_project_steps(token : String) -> Result<Vec<ProjectStep>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("bearer {token}").parse().unwrap());
    let mut projects : Vec<ProjectStep> = Vec::new();
    let client = reqwest::Client::builder().build()?;
    let res = client.get(format!("https://api.kordis.fr/me/nextProjectSteps"))
        .headers(headers.clone())
        .send()
        .await?;
    if res.status() == 200 {
        let res_text = res.text().await?;
        let response: NextProjectStepsWebResult = serde_json::from_str(&res_text)?;
        for project in response.result {
            if let Some(_psp_limit_date) = &project.psp_limit_date {
                projects.push(project.clone());
            }
        }

    }
    Ok(projects)
}