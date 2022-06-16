use chrono::TimeZone;

use crate::raw_types::{delivery_info_response, get_sessions_response};

#[derive(Clone, Debug)]
pub struct Video {
    title: String,
    uploaded_at: chrono::DateTime<chrono::Utc>,
    description: Option<String>,
    length: chrono::Duration,
    thumbnail_path: String,
    id: String,
    direct_mp4: String,

    /// Set with `Client::get_streams`, `None` otherwise
    streams: Option<Streams>,
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

#[derive(Clone, Debug)]
pub struct StreamInfo {
    stream_url: StreamUrl,
}

#[derive(Clone, Debug)]
pub struct Streams {
    streams: Vec<StreamInfo>,
}

impl Video {
    pub fn direct_mp4(&self) -> &str {
        &self.direct_mp4
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn set_streams(&mut self, streams: Streams) {
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
        let uploaded_time: i64 = result.start_time[6..result.start_time.len() - 2]
            .parse()
            .expect("Invalid uploaded time");
        #[allow(clippy::cast_possible_truncation)]
        Self {
            id: result.delivery_id,
            title: result.session_name,
            description: result.result_abstract,
            length: chrono::Duration::milliseconds((result.duration * 1000.0) as i64),
            thumbnail_path: result.thumb_url,
            uploaded_at: chrono::Utc.timestamp(uploaded_time, 0),
            direct_mp4: result.ios_video_url,
            streams: None,
        }
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
        let mut streams = Vec::new();
        for stream in response.delivery.streams {
            // TODO Remove query params and port from URLs if included
            let stream_url = match stream.viewer_media_file_type_name.as_str() {
                "hls" => StreamUrl::Hls(stream.stream_url),
                "mp4" => StreamUrl::Mp4(stream.stream_url),
                stream_type => panic!("Invalid stream type {}", stream_type),
            };
            streams.push(StreamInfo { stream_url });
        }
        Self { streams }
    }
}
