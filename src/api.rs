use reqwest::header::HeaderValue;

use crate::custom_types;
use crate::get_sessions_request;
use crate::get_sessions_response;

pub struct Client {
    host: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(host: String, cookie: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("authority", host.parse().expect("Invalid host"));
        headers.insert(
            "origin",
            format!("https://{}", host).parse().expect("Invalid host"),
        );
        headers.insert(
            "cookie",
            format!(".ASPXAUTH={};", cookie)
                .parse()
                .expect("Invalid cookie"),
        );

        for (header, value) in [
            ("content-type", "application/json; charset=UTF-8"),
            ("x-requested-with", "XMLHttpRequest"),
            ("accept", "*/*"),
            ("sec-fetch-site", "same-origin"),
            ("sec-fetch-mode", "cors"),
            ("sec-fetch-dest", "empty"),
        ] {
            headers.insert(header, HeaderValue::from_static(value));
        }

        Self {
            host,
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn get_folder_from_id(
        &self,
        folder_id: String,
    ) -> Result<custom_types::FolderListing, reqwest::Error> {
        Ok(self
            .client
            .post(format!(
                "https://{}/Panopto/Services/Data.svc/GetSessions",
                self.host
            ))
            .json(&get_sessions_request::Root {
                query_parameters: get_sessions_request::QueryParameters {
                    query: None,
                    sort_column: 1,
                    sort_ascending: true,
                    max_results: 250,
                    page: 0,
                    start_date: None,
                    end_date: None,
                    folder_id,
                    bookmarked: false,
                    get_folder_data: true,
                    is_shared_with_me: false,
                    is_subscriptions_page: false,
                    include_archived: false,
                    include_playlists: false,
                },
            })
            .send()
            .await?
            .json::<get_sessions_response::Root>()
            .await?
            .into())
    }

    pub async fn get_folder_listing(
        &self,
        folder: custom_types::Folder,
    ) -> Result<custom_types::FolderListing, reqwest::Error> {
        self.get_folder_from_id(folder.id().to_string()).await
    }
}
