use reqwest::header;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct WebResult {
    result: Vec<Project>,
}
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


pub async fn get_projects(token : String, years : Vec<u16>) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("bearer {token}").parse().unwrap());
    let mut projects : Vec<Project> = Vec::new();
    let client = reqwest::Client::builder().build()?;
    for year in years {
        let res = client.get(format!("https://api.kordis.fr/me/{year}/projects"))
            .headers(headers.clone())
            .send()
            .await?;
        if res.status() == 200 {
            let res_text = res.text().await?;
            //println!("{}",res_text);
            let response: WebResult = serde_json::from_str(&res_text)?;
            for project in response.result {
                if let Some(_steps) = &project.steps {
                    projects.push(project.clone());
                }
            }
        }

    }
    Ok(projects)
}