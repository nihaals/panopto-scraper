use reqwest::header::HeaderValue;

use crate::{
    custom_types,
    raw_types::{
        delivery_info_request, delivery_info_response, get_sessions_request, get_sessions_response,
    },
};

pub struct Client {
    host: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(host: String, cookie: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("authority", host.parse().expect("invalid host"));
        headers.insert(
            "origin",
            format!("https://{}", host).parse().expect("invalid host"),
        );
        headers.insert(
            "cookie",
            format!(".ASPXAUTH={};", cookie)
                .parse()
                .expect("invalid cookie"),
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
        folder: &custom_types::Folder,
    ) -> Result<custom_types::FolderListing, reqwest::Error> {
        self.get_folder_from_id(folder.id().to_owned()).await
    }

    pub async fn get_streams(&self, video: &mut custom_types::Video) -> Result<(), reqwest::Error> {
        video.set_streams(
            self.client
                .post(format!(
                    "https://{}/Panopto/Pages/Viewer/DeliveryInfo.aspx",
                    self.host
                ))
                .form(&delivery_info_request::Root {
                    delivery_id: video.id().to_owned(),
                    invocation_id: "".to_owned(),
                    is_live_notes: false,
                    refresh_auth_cookie: true,
                    is_active_broadcast: false,
                    is_editing: false,
                    is_kollective_agent_installed: false,
                    is_embed: false,
                    response_type: "json".to_owned(),
                })
                .send()
                .await?
                .json::<delivery_info_response::Root>()
                .await?
                .into(),
        );
        Ok(())
    }

    pub async fn get_video_from_id(
        &self,
        video_id: String,
    ) -> Result<custom_types::Video, reqwest::Error> {
        let delivery_info = self
            .client
            .post(format!(
                "https://{}/Panopto/Pages/Viewer/DeliveryInfo.aspx",
                self.host
            ))
            .form(&delivery_info_request::Root {
                delivery_id: video_id.clone(),
                invocation_id: "".to_owned(),
                is_live_notes: false,
                refresh_auth_cookie: true,
                is_active_broadcast: false,
                is_editing: false,
                is_kollective_agent_installed: false,
                is_embed: false,
                response_type: "json".to_owned(),
            })
            .send()
            .await?
            .json::<delivery_info_response::Root>()
            .await?;
        let folder_id = delivery_info.delivery.session_group_public_id.clone();
        let video = if let Some(folder_id) = folder_id {
            // Get folder-exclusive metadata
            let folder = self.get_folder_from_id(folder_id).await?;
            let mut video = folder
                .videos()
                .iter()
                .find(|v| v.id() == video_id)
                .expect("video not found in parent folder")
                .clone();
            video.set_streams(delivery_info.into());
            video
        } else {
            // Skip folder-exclusive metadata
            // Sets streams already
            delivery_info.clone().into()
        };
        Ok(video)
    }
}
