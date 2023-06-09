use serde::de::DeserializeOwned;

use crate::models::RsTrackResponse;

use super::*;

impl FetchDriver for IgnitionDriver {
    fn fetch<T: DeserializeOwned>(&self) -> ApiResult<T> {
        let url = format!("{}/?{}", Self::FT_URL, self.filters.parse());
        let request = self.client.get(url).build().unwrap();
        
        match self.client.execute(request) {
            Ok(response) => {
                let status = response.status();
                if status.is_success() {
                    // println!("{:#?}", response.json::<Json>().unwrap());
                    // Err(Error::RequestError("".to_string()))
                    return Ok(response.json::<T>().unwrap());
                } else {
                    Err(Error::ApiError(format!("API error: {}", status)))
                }
            },
            Err(error) => Err(Error::RequestError(error.to_string()))
        }
    }
}

// // impl<T: DeserializeOwned> Fetch<T> for IgnitionDriver {
// //     fn fetch(&self) -> ApiResult<T> {
// //         let url = format!("{}/?{}", URL, self.filters.parse());
// //         let request = self.client.get(url).build().unwrap();
        
// //         match self.client.execute(request) {
// //             Ok(response) => {
// //                 let status = response.status();
// //                 if status.is_success() {
// //                     // println!("{:#?}", response.json::<Json>().unwrap());
// //                     // Err(Error::RequestError("".to_string()))
// //                     return Ok(response.json::<T>().unwrap());
// //                 } else {
// //                     Err(Error::ApiError(format!("API error: {}", status)))
// //                 }
// //             },
// //             Err(error) => Err(Error::RequestError(error.to_string()))
// //         }
// //     }
// // }

impl IgnitionDriver {
    pub fn fetch_tracks(&mut self, filters: Option<&IgnitionFilters>) -> ApiResult<RsTrackResponse> {
        if let Some(filters) = filters {
            self.filters = filters.to_owned();
        }
        self.fetch()
    }
}