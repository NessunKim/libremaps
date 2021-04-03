use crate::mw_api;
use crate::schema::markers;
use anyhow::Result;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug)]
pub struct Marker {
    pub id: i32,
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub zoom: i8,
    pub page_id: i32,
    pub page_name: String,
    pub page_revid: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "markers"]
pub struct NewMarker {
    pub name: String,
    pub latitude: f32,
    pub longitude: f32,
    pub zoom: i8,
    pub page_id: i32,
    pub page_name: String,
    pub page_revid: i32,
}

impl Marker {
    pub fn find(
        conn: &MysqlConnection,
        south: f32,
        north: f32,
        west: f32,
        east: f32,
        zoom: i8,
    ) -> Result<Vec<Self>> {
        let west = west + (-west / 360.).round() * 360.;
        let east = east + (-east / 360.).round() * 360.;
        if west < east {
            let results = markers::table
                .filter(markers::latitude.between(south, north))
                .filter(markers::longitude.between(west, east))
                .filter(markers::zoom.le(zoom))
                .limit(100)
                .load::<Self>(conn)?;
            Ok(results)
        } else {
            let results = markers::table
                .filter(markers::latitude.between(south, north))
                .filter(
                    markers::longitude
                        .between(west, 180.)
                        .or(markers::longitude.between(-180., east)),
                )
                .filter(markers::zoom.le(zoom))
                .limit(100)
                .load::<Self>(conn)?;
            Ok(results)
        }
    }

    pub async fn update(conn: &MysqlConnection) -> Result<(), Box<dyn std::error::Error>> {
        let pages = mw_api::get_transcluding_pages().await?;
        let to_update_page_ids = Self::filter_pages_to_check(conn, &pages)?;
        dbg!(&to_update_page_ids);

        diesel::delete(markers::table.filter(markers::page_id.eq_any(&to_update_page_ids)))
            .execute(conn)?;

        let mut new_markers: Vec<NewMarker> = vec![];

        for page_id in to_update_page_ids {
            match mw_api::parse_page(page_id).await {
                Ok(mut markers) => {
                    new_markers.append(&mut markers);
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }

        dbg!(&new_markers);

        diesel::insert_into(markers::table)
            .values(new_markers)
            .execute(conn)?;
        let page_ids = pages.iter().map(|x| x.pageid).collect::<Vec<i32>>();

        Self::remove_invalid_markers(conn, &page_ids)?;

        Ok(())
    }

    /// Returns a list of pages that are needed to be parsed to check if it is needed to be updated.
    /// For each page, if the page is outdated or not in DB, add it to the list.
    fn filter_pages_to_check(
        conn: &MysqlConnection,
        pages: &[mw_api::MwPageInfo],
    ) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
        let mut results: Vec<i32> = vec![];
        for mw_api::MwPageInfo { pageid, lastrevid } in pages {
            let first = markers::table
                .filter(markers::page_revid.eq(lastrevid))
                .first::<Self>(conn)
                .optional()?;
            if first.is_none() && results.len() < 10 {
                results.push(*pageid);
            }
        }
        Ok(results)
    }

    fn remove_invalid_markers(
        conn: &MysqlConnection,
        page_ids: &[i32],
    ) -> Result<(), Box<dyn std::error::Error>> {
        diesel::delete(markers::table.filter(markers::page_id.ne_all(page_ids))).execute(conn)?;
        Ok(())
    }
}
