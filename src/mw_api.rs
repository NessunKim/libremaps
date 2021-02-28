use crate::models::NewMarker;
use actix_web::client::Client;
use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MwContinue {
    gticontinue: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MwQuery {
    pages: HashMap<String, MwPageInfo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MwPageInfo {
    pub pageid: i32,
    pub lastrevid: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MwQueryResponse {
    #[serde(rename = "continue")]
    cnt: Option<MwContinue>,
    query: MwQuery,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MwText {
    #[serde(rename = "*")]
    content: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MwParse {
    title: String,
    revid: i32,
    text: MwText,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MwParseResponse {
    parse: MwParse,
}

/// Returns a list of pages that transclude the template indicating a marker.
pub async fn get_transcluding_pages() -> Result<Vec<MwPageInfo>, Box<dyn std::error::Error>> {
    let mut pages: Vec<MwPageInfo> = vec![];
    let client = Client::default();
    let mut request_url = "https://librewiki.net/api.php?action=query&pageids=164536&generator=transcludedin&gtinamespace=0&gtilimit=100&prop=info&format=json".to_owned();
    loop {
        let mut response = client.get(request_url).send().await?;
        let body = response.body().await?;
        let resp: MwQueryResponse = serde_json::from_slice(&body)?;
        for page in resp.query.pages.values() {
            pages.push(page.clone());
        }

        if let Some(cnt) = resp.cnt {
            request_url = format!("https://librewiki.net/api.php?action=query&pageids=164536&generator=transcludedin&gtinamespace=0&gtilimit=100&gticontinue={}&prop=info&format=json", cnt.gticontinue);
        } else {
            break;
        }
    }
    Ok(pages)
}

pub async fn parse_page(page_id: i32) -> Result<Vec<NewMarker>, Box<dyn std::error::Error>> {
    let client = Client::default();
    let request_url = format!(
        "https://librewiki.net/api.php?action=parse&pageid={}&prop=text|revid&format=json",
        page_id
    );
    let mut response = client.get(request_url).send().await?;
    let body = response.body().await?;
    let resp: MwParseResponse = serde_json::from_slice(&body)?;
    let page_name = resp.parse.title;
    let page_revid = resp.parse.revid;
    let fragment = Html::parse_fragment(&resp.parse.text.content);
    let selector = Selector::parse(".libremaps-marker").map_err(|_| "Parsing html failed")?;
    let mut new_markers: Vec<NewMarker> = vec![];
    for element in fragment.select(&selector) {
        let e = element.value();
        let lng = e
            .attr("data-lng")
            .ok_or_else(|| anyhow!("data-lng is missing"))?
            .parse::<f32>()?;
        let marker = NewMarker {
            name: e
                .attr("data-name")
                .ok_or_else(|| anyhow!("data-name is missing"))?
                .to_owned(),
            latitude: e
                .attr("data-lat")
                .ok_or_else(|| anyhow!("data-lat is missing"))?
                .parse::<f32>()?,
            longitude: lng + (-lng / 360.).round() * 360.,
            zoom: e
                .attr("data-zoom")
                .ok_or_else(|| anyhow!("data-zoom is missing"))?
                .parse::<i8>()?,
            page_id,
            page_name: page_name.clone(),
            page_revid,
        };
        new_markers.push(marker);
    }
    Ok(new_markers)
}
