use reqwest::header;

pub async fn myges_auth(base64: String) -> Result<String, Box<dyn std::error::Error>> {
    //let config = Config::init();
    //let base64_auth = base64::engine::general_purpose::STANDARD.encode(format!("{}:{}",config.myges_username,config.myges_password));

    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Basic {base64}").parse().unwrap());

    let client = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).build()?;
    let res = client.get("https://authentication.kordis.fr/oauth/authorize?response_type=token&client_id=skolae-app")
        .headers(headers)
        .send()
        .await?;
    if res.status() != 302 {
        return Err("Wrong credentials".into());
    }
    if let Some(content_type) = res.headers().get(reqwest::header::LOCATION) {
        let capture_token = regex::Regex::new(r"access_token=(\S+)&token_type").unwrap();
        let cap_token = capture_token.captures(content_type.to_str()?);
        if let Some(cap_token) = cap_token {
            let token=&cap_token[1];
            return Ok(token.to_string());
        }
        else {
            return Err("Wrong credentials".into());
        }
    } else {
        Err("Wrong credentials".into())
    }
}