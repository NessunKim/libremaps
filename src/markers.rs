use actix_web::client::Client;
use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Marker {
    pub id: String,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub zoom: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MwContinue {
    gticontinue: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MwQuery {
    pages: HashMap<String, MwPageInfo>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct MwPageInfo {
    pageid: i32,
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
    text: MwText,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MwParseResponse {
    parse: MwParse,
}

pub static mut MARKERS: Vec<Marker> = vec![];

pub async fn get_candidates() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut to_update: Vec<i32> = vec![];
    let client = Client::default();
    let mut request_url = "https://librewiki.net/api.php?action=query&pageids=164536&generator=transcludedin&gtinamespace=0&gtilimit=100&prop=info&format=json".to_owned();
    loop {
        let mut response = client.get(request_url).send().await?;
        let body = response.body().await?;
        let resp: MwQueryResponse = serde_json::from_slice(&body)?;
        dbg!(&resp);
        to_update.extend(resp.query.pages.values().map(|x| x.pageid));

        if let Some(cnt) = resp.cnt {
            request_url = format!("https://librewiki.net/api.php?action=query&pageids=164536&generator=transcludedin&gtinamespace=0&gtilimit=100&gticontinue={}&prop=info&format=json", cnt.gticontinue);
        } else {
            break;
        }
    }
    Ok(to_update)
}

pub async fn update_markers() -> Result<(), Box<dyn std::error::Error>> {
    let to_update = get_candidates().await?;
    let client = Client::default();
    let mut new_markers: Vec<Marker> = vec![];
    for id in to_update {
        let request_url = format!(
            "https://librewiki.net/api.php?action=parse&pageid={}&prop=text&format=json",
            id
        );
        let mut response = client.get(request_url).send().await?;
        let body = response.body().await?;
        let resp: MwParseResponse = serde_json::from_slice(&body)?;
        let fragment = Html::parse_fragment(&resp.parse.text.content);
        let selector = Selector::parse(".libremaps-marker").unwrap();
        for element in fragment.select(&selector) {
            dbg!(element.value().attr("data-lng"));
            dbg!(element.value().attr("data-lat"));
            let e = element.value();
            let marker = Marker {
                id: format!("{}", id),
                name: e
                    .attr("data-name")
                    .ok_or(anyhow!("data-name is missing"))?
                    .to_owned(),
                latitude: e
                    .attr("data-lat")
                    .ok_or(anyhow!("data-lat is missing"))?
                    .parse::<f32>()?,
                longitude: e
                    .attr("data-lng")
                    .ok_or(anyhow!("data-lng is missing"))?
                    .parse::<f32>()?,
                zoom: e
                    .attr("data-zoom")
                    .ok_or(anyhow!("data-zoom is missing"))?
                    .parse::<i32>()?,
            };
            new_markers.push(marker);
        }
    }
    unsafe {
        MARKERS = new_markers;
    }
    Ok(())
}
