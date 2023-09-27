use actix_web::{web, get, HttpRequest, Result, HttpResponse};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    user: Option<String>,
    password: Option<String>,
}
#[get("/")]
async fn get_calendar_handler(req: HttpRequest) -> Result<HttpResponse> {
    let req_headers = req.headers();
    let basic_auth_header = req_headers.get("Authorization").map(|h| h.to_str().unwrap_or_default().to_string());

    let base64_auth : String;
    let params = web::Query::<User>::from_query(req.query_string()).unwrap();

    if let (Some(user), Some(password)) = (&params.user, &params.password) {
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
    let events = crate::cal::get_events(token).await;
    let cal = crate::cal::events_to_ics(events.unwrap());

    let response = HttpResponse::Ok()
        .append_header((r#"Content-Disposition"#, r#"attachment; filename="myges2ics.ics""#))
        .append_header(("Content-Type","text/calendar; charset=utf-8"))
        .body(cal.to_string());

    Ok(response)

}