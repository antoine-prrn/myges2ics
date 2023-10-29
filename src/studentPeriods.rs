use std::clone;

use reqwest::header;
use crate::config::Config;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct WebResult {
    result: Vec<Semester>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Semester {
    classe: String,
    option: String,
    promotion: String,
    year_id: u16,
}