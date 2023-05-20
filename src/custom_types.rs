use std::str::FromStr;

use chrono::TimeZone;
use url::Url;

use crate::raw_types::{delivery_info_response, get_sessions_response};

#[derive(Clone, Debug)]
struct FolderExclusiveMetadata {
    uploaded_at: chrono::DateTime<chrono::Utc>,
    thumbnail_path: String,
    combined_stream: StreamUrl,
}

#[derive(Clone, Debug)]
pub struct Video {
    title: String,
    description: Option<String>,
    length: chrono::Duration,
    id: String,

    /// Set with `Client::get_streams`, `None` otherwise
    streams: Option<Streams>,

    /// Metadata that can only be accessed by listing the parent folder
    folder_exclusive_metadata: Option<FolderExclusiveMetadata>,
}

#[derive(Clone, Debug)]
pub struct Folder {
    name: String,
    id: String,
}

#[derive(Debug)]
pub struct FolderListing {
    videos: Vec<Video>,
    folders: Vec<Folder>,
}

#[derive(Clone, Debug)]
pub enum StreamUrl {
    Hls(String),
    Mp4(String),
}

impl FromStr for StreamUrl {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with(".mp4") {
            Ok(Self::Mp4(s.to_owned()))
        } else if s.ends_with(".m3u8") {
            Ok(Self::Hls(s.to_owned()))
        } else {
            Err(format!("invalid stream url: {s}"))
        }
    }
}

#[derive(Clone, Debug)]
pub struct StreamInfo {
    stream_url: StreamUrl,
}

#[derive(Clone, Debug)]
pub struct Streams {
    streams: Vec<StreamInfo>,
    combined_streams: Vec<StreamInfo>,
}

impl Video {
    pub fn combined_stream(&self) -> Option<StreamUrl> {
        self.folder_exclusive_metadata
            .clone()
            .map(|metadata| metadata.combined_stream)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn set_streams(&mut self, streams: Streams) {
        assert!(self.streams.is_none(), "streams already set");
        self.streams = Some(streams);
    }
}

impl Folder {
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl FolderListing {
    pub fn videos(&self) -> &[Video] {
        &self.videos
    }

    pub fn folders(&self) -> &[Folder] {
        &self.folders
    }
}

impl From<get_sessions_response::Result> for Video {
    fn from(result: get_sessions_response::Result) -> Self {
        let uploaded_time_unix_ms: i64 = result.start_time[6..result.start_time.len() - 2]
            .parse()
            .expect("invalid uploaded time");
        #[allow(clippy::cast_possible_truncation)]
        Self {
            id: result.delivery_id,
            title: result.session_name,
            description: result.result_abstract,
            length: chrono::Duration::milliseconds((result.duration * 1000.0) as i64),
            folder_exclusive_metadata: Some(FolderExclusiveMetadata {
                uploaded_at: chrono::Utc.timestamp_millis(uploaded_time_unix_ms),
                thumbnail_path: result.thumb_url,
                combined_stream: result.ios_video_url.parse().expect("invalid stream url"),
            }),
            streams: None,
        }
    }
}

impl From<delivery_info_response::Root> for Video {
    fn from(delivery_info: delivery_info_response::Root) -> Self {
        let mut video = {
            let delivery = delivery_info.delivery.clone();
            #[allow(clippy::cast_possible_truncation)]
            Self {
                id: delivery.public_id,
                title: delivery.session_name,
                description: delivery.session_abstract,
                length: chrono::Duration::milliseconds((delivery.duration * 1000.0) as i64),
                folder_exclusive_metadata: None,
                streams: None,
            }
        };
        video.set_streams(delivery_info.into());
        video
    }
}

impl From<get_sessions_response::Subfolder> for Folder {
    fn from(subfolder: get_sessions_response::Subfolder) -> Self {
        Self {
            id: subfolder.id,
            name: subfolder.name,
        }
    }
}

impl From<get_sessions_response::Root> for FolderListing {
    fn from(response: get_sessions_response::Root) -> Self {
        let mut videos = Vec::new();
        for result in response.d.results {
            videos.push(Video::from(result));
        }
        let mut folders = Vec::new();
        for subfolder in response.d.subfolders {
            folders.push(Folder::from(subfolder));
        }
        Self { videos, folders }
    }
}

impl From<delivery_info_response::Root> for Streams {
    fn from(response: delivery_info_response::Root) -> Self {
        let clean_url = |url: String| {
            let mut url = Url::parse(&url).expect("invalid url");
            url.set_query(None);
            url.set_port(None).unwrap();
            url.to_string()
        };
        let map = |stream: delivery_info_response::Stream| StreamInfo {
            stream_url: match stream.viewer_media_file_type_name.as_str() {
                "hls" => StreamUrl::Hls(clean_url(stream.stream_url)),
                "mp4" => StreamUrl::Mp4(clean_url(stream.stream_url)),
                stream_type => panic!("invalid stream type {}", stream_type),
            },
        };

        let streams = response.delivery.streams.into_iter().map(map).collect();
        let combined_streams = response
            .delivery
            .podcast_streams
            .into_iter()
            .map(map)
            .collect();
        Self {
            streams,
            combined_streams,
        }
    }
}
