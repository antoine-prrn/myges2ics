use actix_web::{web, get, HttpRequest, Result, HttpResponse};
use serde_derive::Deserialize;

use crate::cal::projects_to_ics;

#[derive(Deserialize, Debug)]
struct User {
    user: Option<String>,
    password: Option<String>,
    base64: Option<String>,
}

#[get("/")]
async fn get_calendar_handler(req: HttpRequest) -> Result<HttpResponse> {
    let req_headers = req.headers();
    let basic_auth_header = req_headers.get("Authorization").map(|h| h.to_str().unwrap_or_default().to_string());

    let base64_auth : String;
    let params = web::Query::<User>::from_query(req.query_string()).unwrap();
    if let Some(base64) = &params.base64 {
        base64_auth = base64.to_string();
    }
    else if let (Some(user), Some(password)) = (&params.user, &params.password) {
        base64_auth = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, format!("{}:{}",user,password));
    }
    else if let Some(basic_auth_header) = basic_auth_header {
        base64_auth = basic_auth_header.split(" ").collect::<Vec<&str>>()[1].to_string();
    }
    else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    
    let token = match crate::auth::myges_auth(base64_auth).await {
        Ok(result) => result,
        Err(_err) => {
            return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
        }
    };
    let events = crate::cal::get_events(token.clone()).await;
    let mut cal = crate::cal::events_to_ics(events.unwrap());

    let current_year = current_year();
    let mut years : Vec<u16> = Vec::new();
    years.push(current_year);
    years.push(current_year-1);
    
    let projects = crate::projects::get_projects(token, years).await;
    cal = projects_to_ics(projects?, cal);
    let response = HttpResponse::Ok()
        .append_header((r#"Content-Disposition"#, r#"attachment; filename="myges2ics.ics""#))
        .append_header(("Content-Type","text/calendar; charset=utf-8"))
        .body(cal.to_string());

    Ok(response)

}

pub fn current_year() -> u16 {
    let now = std::time::SystemTime::now();
    let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
    let current_year = since_the_epoch.as_secs() / (365 * 24 * 60 * 60) + 1970;
    return current_year as u16;
}