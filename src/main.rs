
use std::{thread::{self, Thread}, time::Duration};

use clap::{Parser, Args, Subcommand};
use easydev::builder::*;
use rs2014_dl::{Ignition, IgnitionFilters, models::{track::RsTrack, RsTrackResponse}, Download, DownloadState};


#[derive(Subcommand, Default, Debug, Clone, Copy, PartialEq)]
enum CliAction {
    Download,
    #[default]
    Fetch,
}

#[derive(Default, Debug, Clone, Parser)]
struct CliArgs {
    #[clap(flatten)]
    filters: Option<IgnitionFilters>,

    #[clap(subcommand)]
    action: Option<CliAction>,

    // #[clap(flatten)]
    // fetch_options: FetchOptions,
}

fn main() {
    let args = CliArgs::parse();
    let mut fetcher = Ignition::from_env().with_filters(args.filters.unwrap_or_default());

    match args.action.unwrap_or_default() {
        CliAction::Fetch => { fetch(&mut fetcher); },
        CliAction::Download =>{ download(&mut fetcher); },
    }
    // download_all(fetcher);
    // if filters.is_empty() {
    //     panic!("At least 'search' filter must be specified\nTry using flag --search \"Metallica\"");
    // }

}

// #[derive(WithBuilder, Args, Debug, Clone, Copy)]
// pub struct FetchOptions {
//     start: usize,
//     length: usize,
//     repeat: usize,
//     // search: Option<String>,
// }
// impl Default for FetchOptions {
//     fn default() -> Self {
//         Self { start: 0, length: 50, repeat: 1 }
//     }
// }

#[derive(WithBuilder, Debug, Clone, Copy)]
pub struct DownloadStats {
    c_downloads: usize,
    c_skipped: usize,
    c_failed: usize,
    c_filtered: usize,
}
impl DownloadStats {
    pub fn update_add(
        mut self,
        c_downloads: usize,
        c_skipped: usize,
        c_failed: usize,
        c_filtered: usize,
    ) -> Self {
        self.c_downloads += c_downloads;
        self.c_skipped += c_skipped;
        self.c_failed += c_failed;
        self.c_filtered += c_filtered;
        self
    }

    pub fn total(&self) -> usize {
        self.c_downloads + self.c_skipped + self.c_failed + self.c_filtered
    }
}

pub fn fetch(mut fetcher: &mut Ignition) -> RsTrackResponse {
    let IgnitionFilters { start, length, .. } = fetcher.filters();
    let mut response = match fetcher.fetch_tracks(None) {
        Ok(resp) => resp,
        Err(err) => panic!("{err}"),
    };

    let mut combined_data: Vec<RsTrack> = response.data.clone();
    let mut fetched_records_count = response.data.len();
    while fetched_records_count == IgnitionFilters::MAX_LENGTH && combined_data.len() < length && fetcher.filters().start < response.recordsTotal {
        let mut filters = fetcher.filters();
        filters.start += IgnitionFilters::MAX_LENGTH;
        
        println!("Fetching records {}-{}...", filters.start, filters.start+IgnitionFilters::MAX_LENGTH);
        fetcher = fetcher.set_filters(filters);
        let rs_response = match fetcher.fetch_tracks(None) {
            Ok(resp) => resp,
            Err(err) => panic!("{err}"),
        };
        fetched_records_count = rs_response.data.len();
        combined_data.extend(rs_response.data);
    }

    println!("\n------------------------\nList of all fetched tracks [{}]:\n------------------------", &combined_data.len());

    let mut i = 0usize;
    for track in &combined_data {
        let RsTrack { artist, title, downloads, duration,  version, author, updated_at,  .. } = track;

        i+= 1;
        println!(
            "{i}: [{}] {} - {title} v{version} (by {}) - ={downloads}= [{updated_at}]",
            duration.to_owned().unwrap_or("".to_string()), artist.name, author.name
        )
    }

    response.data = combined_data;
    response
    // let IgnitionFilters { start, length, .. } = fetcher.filters();
    // let repeat = if repeat == 0 { 1 } else { repeat };
    // let start = if start > 65000 { 65000 } else { start };
    // let length = if length > 500 { 500 } else { length };

    // fetcher.filters().set_start(start);
    // fetcher.filters().set_length(length);

}

pub fn download(mut fetcher: &mut Ignition) {
    let RsTrackResponse { mut data, .. } = fetch(fetcher);

    let mut filtered_tracks: Vec<RsTrack> = vec![];
    for track in data.iter_mut() {
        let RsTrack { artist, title, duration, file_pc_link,  .. } = track;
        if duration.is_none() || duration.to_owned().unwrap().is_empty() {
            // eprintln!("[WARN] Duration not specified in '{} - {}'", title, artist.name);
            continue;
        }

        let Some(download_link) = file_pc_link.to_owned() else {
            // eprintln!("[ERROR] No link to download '{} - {}'", title, artist.name);
            continue;
        };
        // println!("[DEBUG] {} - {} [{}]", artist.name, title, download_link);
        match fetcher.parse_download_url(&download_link) {
            Ok(url) => {
                track.file_pc_link = Some(url);
                filtered_tracks.push(track.clone());
            },
            Err(error) => {
                // eprintln!("[ERROR] {}", error);
                continue;
            }
        }
    }

    println!("\n------------------------\nList of all tracks to download:\n------------------------");
    let mut i = 0usize;
    for track in &filtered_tracks {
        let RsTrack { artist, title, downloads, duration,  version, author, updated_at,  .. } = track;

        i+= 1;
        println!(
            "{i}: [{}] {} - {title} v{version} (by {}) - ={downloads}= [{updated_at}]",
            duration.to_owned().unwrap_or("".to_string()), artist.name, author.name
        )
    }

    println!("\n---------------\nPreparing to download {}/{} tracks from Ignition...\n----------------\n", filtered_tracks.len(), data.len());
    thread::sleep(Duration::from_secs(3));

    let mut download_stats = DownloadStats::new().build();
    download_stats.c_filtered = &data.len() - filtered_tracks.len();

    for RsTrack { artist, title, file_pc_link, version, author, downloads, .. } in &filtered_tracks {
        let Some(download_link) = file_pc_link else { continue; };
        let file_name = format!("{}-{}_[{}]_v{}_p.psarc", artist.name, title, author.name, version);

        print!("[INFO] Downloading '{} - {} v{} [D:{}, A:{}]'...", artist.name, title, version, downloads, author.name);
        // println!("[DEBUG] Url: {}", download_link);
        match fetcher.download(download_link, file_name) {
            Ok(status) => {
                match status {
                    DownloadState::Downloaded => {
                        print!("DOWNLOADED\n");
                        download_stats.c_downloads+=1;
                    },
                    DownloadState::Skipped => {
                        print!("ALREADY_EXISTS\n");
                        download_stats.c_skipped+=1;
                    }
                }
            },
            Err(e) => {
                eprintln!("{e}");
                download_stats.c_failed+=1;
            }
        }
    }

    let DownloadStats { c_downloads, c_skipped, c_failed, c_filtered} = download_stats;
    println!("
    \n------------------\nTotal Songs Processed: {}\nDownloaded: {c_downloads}\nSkipped: {c_skipped}\nErrors: {c_failed}\nFiltered: {c_filtered}\n------------------------------",
    download_stats.total()
    );

    // download_stats
}

// pub fn download_once(mut fetcher: Ignition, opts: Option<DownloadOptions>) {

//     println!("About to download {} of total {} records. Click Ctrl+C if you wish to cancel.", data.len(), response.recordsTotal);
//     let mut timeout = 3;
//     while timeout > 0 {
//         timeout-=1;
//         thread::spawn(move || {
//             println!("{}", timeout+1);
//             thread::sleep(Duration::from_secs(1));
//         }).join().unwrap();
//     }
//     let (mut c_down, mut c_skip, mut c_err) = (0usize, 0usize, 0usize);
//     for RsTrack { artist, title, file_pc_link, version, .. } in data {
//         let file_name = format!("{}-{}_v{}.psarc", artist.name.replace(r"\s", "_"), title.replace(r"\s", "_"), version);
//         println!("\n{} - {} [{}]", artist.name, title, file_pc_link.clone().unwrap());

//         match fetcher.download(&file_pc_link.unwrap(), file_name) {
//             Ok(status) => {
//                 match status {
//                     rs2014::DownloadState::Downloaded => {
//                         println!("[SUCCESS: DOWNLOADED]");
//                         c_down+=1;
//                     },
//                     rs2014::DownloadState::Skipped => {
//                         println!("[SUCCESS: ALREADY EXISTS]");
//                         c_skip+=1;
//                     }
//                 }
//             },
//             Err(e) => {
//                 eprintln!("{e}");
//                 c_err+=1;
//             }
//         }
//     }

//     println!("
// ------------------\nTotal Songs Processed: {}\nDownloaded: {c_down}\nSkipped: {c_skip}\nErrors: {c_err}",
//         c_down + c_skip + c_err
//     );
// }

pub fn download_all(fetcher: Ignition) {
    // let download_opts = FetchOptions::new()
    //     .with_repeat(2000)
    //     .with_length(500)
    //     .build();
    // let stats = download(fetcher, download_opts);

//     let DownloadStats { c_downloads, c_skipped, c_failed } = stats;
//     let c_total = stats.total();
//     println!("
// ------------------\nTotal Songs Processed: {c_total}\nDownloaded: {c_downloads}\nSkipped: {c_skipped}\nErrors: {c_failed}"
//     );
}