use ics::properties::{Location, Summary, Geo, LastModified, Transp, Sequence};
use reqwest::header;
use crate::config::Config;
use serde::{Deserialize, Serialize};
use chrono::{Utc, TimeZone, LocalResult};
use std::time::{SystemTime, UNIX_EPOCH};
use ics::{self, ICalendar, properties::{DtStart, DtEnd, Color, Description}};
use rand::Rng;
#[derive(Deserialize, Debug)]
struct WebResult {
    result: Vec<Event>,
}
#[derive(Debug, Deserialize)]
pub struct Event {
    rooms: Option<Vec<Room>>,
    #[serde(rename = "type")]
    event_type: String,
    modality: String,
    start_date: u128,
    end_date: u128,
    name: String,
    teacher: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Room {
    name: String,
    floor: String,
    campus: String,
    color: String,
    latitude: String,
    longitude: String,
}


pub async fn get_events(token : String) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
    let config = Config::init();
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("bearer {token}").parse().unwrap());

    let client = reqwest::Client::builder().build()?;
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() - config.days_before_to_fetch as u128 *86400000;

    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() + config.days_to_fetch as u128 *86400000;

    let res = client.get(format!("https://api.kordis.fr/me/agenda?start={start_time}&end={end_time}"))
        .headers(headers)
        .send()
        .await?;
    let res_text = res.text().await?;
    let response: WebResult = serde_json::from_str(&res_text).expect("Erreur lors de la désérialisation du JSON");
    
    Ok(response.result)
}

pub fn events_to_ics(events: Vec<Event>) -> ICalendar<'static> {

    let mut calendar = ICalendar::new("2.0","-//myges2ics//v1.0//FR");
    for event in events {

        let mut cal_event = ics::Event::new(generate_uid(13),Utc::now().format("%Y%m%dT%H%M%SZ").to_string());
        cal_event.push(DtStart::new(format!("{}",milliseconds_to_iso8601(&event.start_date.to_string()).unwrap())));
        cal_event.push(DtEnd::new(format!("{}",milliseconds_to_iso8601(&event.end_date.to_string()).unwrap())));
        cal_event.push(Description::new(format!("{} {} ({})",event.teacher, event.event_type, event.modality)));
        cal_event.push(LastModified::new(Utc::now().format("%Y%m%dT%H%M%SZ").to_string()));
        cal_event.push(Transp::new("OPAQUE"));
        cal_event.push(Sequence::new("0"));
        cal_event.push(Summary::new(event.name));
        if let Some(rooms) = &event.rooms {
            for room in rooms{
                cal_event.push(Geo::new(format!("{};{}",&room.latitude, &room.longitude)));
                cal_event.push(Location::new(format!("{} - {} ({})",&room.campus, &room.name, &room.floor)));
                cal_event.push(Color::new(format!("{}", &room.color)));
            }
        }
        calendar.add_event(cal_event);
    }
    //calendar.write(writer)
    //calendar.save_file("myges.ics").unwrap();
    return calendar;
    
}
fn milliseconds_to_iso8601(milliseconds_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let milliseconds: i64 = milliseconds_str.parse()?;
    
    let dt_result = Utc.timestamp_millis_opt(milliseconds);
    let dt = match dt_result {
        LocalResult::Single(dt) => dt,
        _ => return Err("Datetime conversion failed".into()),
    };
    Ok(dt.format("%Y%m%dT%H%M%SZ").to_string())
}

fn generate_uid(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let characters = "0123456789abcdef";
    let uid: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..characters.len());
            characters.chars().nth(idx).unwrap()
        })
        .collect();
    uid
}