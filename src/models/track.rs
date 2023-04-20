
use crate::*;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub id: usize,
    pub name: String,
    pub group: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: usize,
    pub name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RsTrack {
    pub addBtn: String,
    pub admin_note: Option<String>,
    pub album: String,
    pub album_art_url: String,
    pub albumName: String,
    pub alt_bass: String,
    pub alt_lead: String,
    pub alt_rhythm: String,
    pub arrangements: String,
    pub artist: Artist,
    pub artist_id: usize,
    pub artistName: String,
    pub artistPreferred: bool,
    pub audio_type: String,
    pub author: Author,
    pub author_id: usize,
    pub author_note: String,
    pub bass: String,
    pub chord_chart: usize,
    pub collected: bool,
    pub collected_cdlcs_count: usize,
    pub comments_count: usize,
    pub created_at: String,
    pub differentTunings: bool,
    pub discussionID: Option<usize>,
    pub downloads: usize,
    pub downloads_seven_day: usize,
    pub downloads_thirty_day: usize,
    pub duration: Option<String>,
    pub file_mac_link: Option<String>,
    pub file_pc_link: Option<String>,
    pub has_lyrics: usize,
    pub hiding: bool,
    pub id: usize,
    pub inSaved: bool,
    pub is_disabled: usize,
    pub is_official: usize,
    pub is_rsplus: usize,
    pub lead: String,
    pub memberName: String,
    pub music_video_url: String,
    pub owned: bool,
    pub parts: String,
    pub playthrough_video_url: String,
    pub platforms: String,
    pub require_capo_lead: usize,
    pub require_capo_rhythm: usize,
    pub require_five_bass: usize,
    pub require_heavy_gauge: usize,
    pub require_other_instruments: usize,
    pub require_seven_guitar: usize,
    pub require_six_bass: usize,
    pub require_slide_lead: usize,
    pub require_slide_rhythm: usize,
    pub require_true_tuning: usize,
    pub require_twelve_guitar: usize,
    pub require_whammy_bar: usize,
    pub required: String,
    pub reports_count: usize,
    pub rhythm: String,
    pub title: String,
    pub titleName: String,
    pub tunings: String,
    pub updated_at: String,
    pub version: f32,
    pub year: Option<usize>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RsTrackResponse {
    pub draw: usize,
    pub recordsTotal: usize,
    pub recordsFiltered: usize,
    pub data: Vec<RsTrack>,
}

impl LoadFromCache for RsTrackResponse {
    type Output = Self;
    fn load_from_cache() -> crate::ApiResult<Self::Output> {
        
        // if let Err(error) = serde_json::from_reader(rdr) {
        //     return Err(Error::FileError(format!("Reading file failed: {}", error)));
        // }
        Ok(Default::default())
    }
}

impl SaveToCache for RsTrackResponse {
    fn save_to_cache() -> crate::ApiResult<()> {
        Ok(())
    }
}