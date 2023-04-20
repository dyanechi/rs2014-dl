use clap::Args;

use super::*;


#[derive(Debug, Clone)]
pub struct Ignition {
    pub(crate) auth: IgnitionAuth,
    pub(crate) client: reqwest::blocking::Client,
    pub(crate) filters: IgnitionFilters,
}
impl Ignition {
    pub fn new(auth: IgnitionAuth) -> Ignition {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json, text/javascript, */*; q=0.01".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
        headers.insert("Accept-Language", "en-US,en;q=0.5".parse().unwrap());
        headers.insert("Connection", "keep-alive".parse().unwrap());
        headers.insert("DNT", "1".parse().unwrap());
        headers.insert("Host", "ignition4.customsforge.com".parse().unwrap());
        headers.insert("Referer", "https://ignition4.customsforge.com/".parse().unwrap());
        headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
        headers.insert("Sec-Fetch-Site", "same-origin".parse().unwrap());
        headers.insert("Sec-GPC", "1".parse().unwrap());
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        headers.insert("X-CSRF-TOKEN", auth.csrf_token.parse().unwrap());
        headers.insert(COOKIE, auth.cookie.parse().unwrap());

        let client = reqwest::blocking::Client::builder()
            .brotli(true)
            .default_headers(headers)
            .build()
            .unwrap();

        let filters = IgnitionFilters::default();
        Ignition { auth, client, filters }
    }
    pub fn from_env() -> Ignition {
        Ignition::new(IgnitionAuth::from_env())
    }

    pub fn filters(&self) -> IgnitionFilters {
        self.filters.clone()
    }

    pub fn with_filters(mut self, filters: IgnitionFilters) -> Self {
        self.filters = filters.clone(); self
    }
    pub fn set_filters(&mut self, filters: IgnitionFilters) -> &mut Self {
        self.filters = filters.clone(); self
    }
}

#[derive(Debug, Clone)]
pub struct IgnitionAuth {
    csrf_token: String,
    cookie: String,
}
impl IgnitionAuth {
    pub fn new(csrf_token: String, cookie: String) -> IgnitionAuth {
        IgnitionAuth { csrf_token, cookie }
    }
    pub fn from_env() -> IgnitionAuth {
        let csrf_token = dotenvy::var("CSRF_TOKEN").unwrap();
        let cookie = dotenvy::var("COOKIE").unwrap();
        IgnitionAuth { csrf_token, cookie }
    }
}


#[derive(Args, Clone, Debug, Default, WithBuilder)]
pub struct IgnitionFilters {
    #[arg(short, long)]
    title: Option<String>,
    #[arg(short, long)]
    artist: Option<String>,
    #[arg(short, long)]
    album: Option<String>,
    #[arg(short, long)]
    tuning: Option<String>,
    #[arg(short, long)]
    preferred_platform: Option<String>,
    #[arg(short, long)]
    start_year: Option<String>,
    #[arg(short, long)]
    search: Option<String>,
    #[arg(short, long)]
    end_year: Option<String>,
    #[arg(short, long)]
    preferred: Option<String>,
    #[arg(short, long)]
    lovedCreators: Option<String>,
    #[arg(short, long)]
    lovedCdlcs: Option<String>,
    #[arg(short, long)]
    official: Option<String>,
    #[arg(short, long)]
    rsplus: Option<String>,
    #[arg(short, long)]
    disable: Option<String>,
    #[arg(short, long)]
    notdisabled: Option<String>,
    #[arg(short, long)]
    hidden: Option<String>,
    #[arg(short, long, default_value_t = 0)]
    pub start: usize,
    #[arg(short, long, default_value_t = 25)]
    pub length: usize,
}

impl IgnitionFilters {
    pub const MAX_LENGTH: usize = 500;
    pub fn is_empty(&self) -> bool {
        if self.title.is_none()
        && self.artist.is_none()
        && self.album.is_none()
        && self.tuning.is_none()
        && self.preferred_platform.is_none()
        && self.start_year.is_none()
        && self.search.is_none()
        && self.end_year.is_none()
        && self.preferred.is_none()
        && self.lovedCreators.is_none()
        && self.lovedCdlcs.is_none()
        && self.official.is_none()
        && self.rsplus.is_none()
        && self.disable.is_none()
        && self.notdisabled.is_none()
        && self.hidden.is_none() {
            return true
        }
        false
    }

    pub fn parse(&self) -> String {
        let mut filter_map = HashMap::new();
        let mut filter_string = String::from("draw=1");

        filter_map.insert("columns[0][data]", "addBtn".to_string());
        filter_map.insert("columns[0][searchable]", "false".to_string());
        filter_map.insert("columns[0][orderable]", "false".to_string());
        filter_map.insert("columns[1][data]", "artistName".to_string());
        filter_map.insert("columns[1][name]", "artist.name".to_string());
        filter_map.insert("columns[2][data]", "titleName".to_string());
        filter_map.insert("columns[2][name]", "title".to_string());
        filter_map.insert("columns[3][data]", "albumName".to_string());
        filter_map.insert("columns[3][name]", "album".to_string());
        filter_map.insert("columns[4][data]", "year".to_string());
        filter_map.insert("columns[5][data]", "duration".to_string());
        filter_map.insert("columns[5][orderable]", "false".to_string());
        filter_map.insert("columns[6][data]", "tunings".to_string());
        filter_map.insert("columns[6][searchable]", "false".to_string());
        filter_map.insert("columns[6][orderable]", "false".to_string());
        filter_map.insert("columns[7][data]", "version".to_string());
        filter_map.insert("columns[7][searchable]", "false".to_string());
        filter_map.insert("columns[7][orderable]", "false".to_string());
        filter_map.insert("columns[8][data]", "memberName".to_string());
        filter_map.insert("columns[8][name]", "author.name".to_string());
        filter_map.insert("columns[9][data]", "created_at".to_string());
        filter_map.insert("columns[9][searchable]", "false".to_string());
        filter_map.insert("columns[10][data]", "updated_at".to_string());
        filter_map.insert("columns[10][searchable]", "false".to_string());
        filter_map.insert("columns[11][data]", "downloads".to_string());
        filter_map.insert("columns[11][searchable]", "false".to_string());
        filter_map.insert("columns[12][data]", "parts".to_string());
        filter_map.insert("columns[12][orderable]", "false".to_string());
        filter_map.insert("columns[13][data]", "platforms".to_string());
        filter_map.insert("columns[14][data]", "file_pc_link".to_string());
        filter_map.insert("columns[14][searchable]", "false".to_string());
        filter_map.insert("columns[15][data]", "file_mac_link".to_string());
        filter_map.insert("columns[15][searchable]", "false".to_string());
        filter_map.insert("columns[16][data]", "artist.name".to_string());
        filter_map.insert("columns[17][data]", "title".to_string());
        filter_map.insert("columns[18][data]", "album".to_string());
        filter_map.insert("columns[19][data]", "author.name".to_string());
        filter_map.insert("columns[20][data]", "discussionID".to_string());
        filter_map.insert("order[0][column]", "10".to_string());
        filter_map.insert("order[0][dir]", "desc".to_string());

        let length = self.length.clamp(25, Self::MAX_LENGTH);
        filter_map.insert("length", length.to_string());
        filter_map.insert("start", self.start.to_string());
        filter_map.insert("search[value]", self.search.to_owned().unwrap_or_default());
        filter_map.insert("table_filter_album", "".to_string());
        filter_map.insert("table_filter_artist", self.artist.to_owned().unwrap_or_default());
        filter_map.insert("filter_title", self.title.to_owned().unwrap_or_default());
        filter_map.insert("filter_album", self.album.to_owned().unwrap_or_default());
        filter_map.insert("filter_tuning[]", self.tuning.to_owned().unwrap_or_default());
        filter_map.insert("filter_preferred_platform", self.preferred_platform.to_owned().unwrap_or_default());
        filter_map.insert("filter_start_year", self.start_year.to_owned().unwrap_or_default());
        filter_map.insert("filter_end_year", self.end_year.to_owned().unwrap_or_default());
        filter_map.insert("filter_preferred", self.preferred.to_owned().unwrap_or_default());
        filter_map.insert("filter_lovedCreators", self.lovedCreators.to_owned().unwrap_or_default());
        filter_map.insert("filter_lovedCdlcs", self.lovedCdlcs.to_owned().unwrap_or_default());
        filter_map.insert("filter_official", self.official.to_owned().unwrap_or_default());
        filter_map.insert("filter_rsplus", self.rsplus.to_owned().unwrap_or_default());
        filter_map.insert("filter_disable", self.disable.to_owned().unwrap_or_default());
        filter_map.insert("filter_notdisabled", self.notdisabled.to_owned().unwrap_or_default());
        filter_map.insert("filter_hidden", self.hidden.to_owned().unwrap_or_default());

        for (k, v) in filter_map.iter() {
            if v.is_empty() { continue; };
            filter_string.push_str(&format!("&{}={}", k, v))
        }

        filter_string
    }

    pub fn set_start(mut self, start: usize) -> Self { self.start = start; self }
    pub fn set_length(mut self, length: usize) -> Self { self.length = length; self }
    pub fn set_search(mut self, search: impl Into<String>) -> Self { self.search = Some(search.into()); self }
}