use crate::conf::conf;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct FindPartyRequest<'a> {
    query: &'a str,
    // Можно добавить другие параметры, например count, branch_type, type и т.п.
}

pub async fn find_party(query: &str) -> Result<FindPartyResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = "https://suggestions.dadata.ru/suggestions/api/4_1/rs/findById/party";

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Token {}", *conf::DADATA_API_KEY))?,
    );

    let request_body = FindPartyRequest { query };

    let resp = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?
        .error_for_status()?
        .json::<FindPartyResponse>()
        .await?;

    Ok(resp)
}

#[derive(Deserialize, Debug)]
pub struct FindPartyResponse {
    pub suggestions: Vec<Suggestion>,
}

#[derive(Deserialize, Debug)]
pub struct Suggestion {
    #[allow(dead_code)]
    pub value: String,
    #[allow(dead_code)]
    pub unrestricted_value: String,
    pub data: OrgData,
}

#[derive(Deserialize, Debug)]
pub struct OrgData {
    pub inn: Option<String>,
    pub ogrn: Option<String>,
    pub management: Option<Management>,
    pub name: Option<Name>,
    pub opf: Option<Opf>,
    pub address: Option<Address>,
}

#[derive(Deserialize, Debug)]
pub struct Management {
    pub name: Option<String>,
    pub post: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Name {
    pub full_with_opf: Option<String>,
    pub short_with_opf: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Opf {
    pub full: Option<String>,
    pub short: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Address {
    pub value: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ExtractedOrgInfo {
    pub short_name_with_opf: Option<String>,
    pub inn: Option<String>,
    pub fact_address: Option<String>,
    pub legal_address: Option<String>,
    pub management_post: Option<String>,
    pub management_name: Option<String>,
    pub ogrn: Option<String>,
    pub full_name_with_opf: Option<String>,
    pub opf_full: Option<String>,
    pub opf_short: Option<String>,
}

impl FindPartyResponse {
    pub fn extract_first_organization(&self) -> Option<ExtractedOrgInfo> {
        let suggestion = self.suggestions.get(0)?;

        Some(ExtractedOrgInfo {
            short_name_with_opf: suggestion
                .data
                .name
                .as_ref()
                .and_then(|n| n.short_with_opf.clone()),
            inn: suggestion.data.inn.clone(),
            fact_address: None, // В DaData нет отдельного поля для фактического адреса
            legal_address: suggestion
                .data
                .address
                .as_ref()
                .and_then(|a| a.value.clone()),
            management_post: suggestion
                .data
                .management
                .as_ref()
                .and_then(|m| m.post.clone()),
            management_name: suggestion
                .data
                .management
                .as_ref()
                .and_then(|m| m.name.clone()),
            ogrn: suggestion.data.ogrn.clone(),
            full_name_with_opf: suggestion
                .data
                .name
                .as_ref()
                .and_then(|n| n.full_with_opf.clone()),
            opf_full: suggestion.data.opf.as_ref().and_then(|o| o.full.clone()),
            opf_short: suggestion.data.opf.as_ref().and_then(|o| o.short.clone()),
        })
    }
}
