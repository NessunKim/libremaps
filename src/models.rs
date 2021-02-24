use crate::mw_api;
use crate::schema::markers;
use anyhow::Result;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug)]
pub struct Marker {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub zoom: i16,
    pub page_id: i32,
    pub page_name: String,
    pub page_revid: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "markers"]
pub struct NewMarker {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub zoom: i16,
    pub page_id: i32,
    pub page_name: String,
    pub page_revid: i32,
}

impl Marker {
    pub fn find(
        conn: &PgConnection,
        south: f64,
        north: f64,
        west: f64,
        east: f64,
        zoom: i16,
    ) -> Result<Vec<Self>> {
        let results = markers::table
            .filter(markers::latitude.between(south, north))
            .filter(markers::longitude.between(west, east))
            .filter(markers::zoom.le(zoom))
            .load::<Self>(conn)?;
        Ok(results)
    }

    pub async fn update(conn: &PgConnection) -> Result<(), Box<dyn std::error::Error>> {
        let pages = mw_api::get_transcluding_pages().await?;
        dbg!(&pages);
        let to_update_page_ids = Self::filter_pages_to_check(conn, &pages)?;
        dbg!(&to_update_page_ids);

        diesel::delete(markers::table.filter(markers::page_id.eq_any(&to_update_page_ids)))
            .execute(conn)?;

        let mut new_markers: Vec<NewMarker> = vec![];

        for page_id in to_update_page_ids {
            let mut markers = mw_api::parse_page(page_id).await?;
            new_markers.append(&mut markers);
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
        conn: &PgConnection,
        pages: &[mw_api::MwPageInfo],
    ) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
        let mut results: Vec<i32> = vec![];
        for mw_api::MwPageInfo { pageid, lastrevid } in pages {
            let first = markers::table
                .filter(markers::page_revid.eq(lastrevid))
                .first::<Self>(conn)
                .optional()?;
            if first.is_none() {
                results.push(*pageid);
            }
        }
        Ok(results)
    }

    fn remove_invalid_markers(
        conn: &PgConnection,
        page_ids: &[i32],
    ) -> Result<(), Box<dyn std::error::Error>> {
        diesel::delete(markers::table.filter(markers::page_id.ne_all(page_ids))).execute(conn)?;
        Ok(())
    }
}
