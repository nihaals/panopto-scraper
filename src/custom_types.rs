use chrono::TimeZone;

use crate::get_sessions_response;

#[derive(Debug)]
pub struct Video {
    title: String,
    uploaded_at: chrono::DateTime<chrono::Utc>,
    description: String,
    length: chrono::Duration,
    thumbnail_path: String,
    id: String,
}

#[derive(Debug)]
pub struct Folder {
    name: String,
    id: String,
}

#[derive(Debug)]
pub struct FolderListing {
    videos: Vec<Video>,
    folders: Vec<Folder>,
}

impl Folder {
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl From<get_sessions_response::Result> for Video {
    fn from(result: get_sessions_response::Result) -> Self {
        let uploaded_time: i64 = result.start_time[6..result.start_time.len() - 2]
            .parse()
            .expect("Invalid uploaded time");
        Self {
            id: result.delivery_id,
            title: result.session_name,
            description: result.result_abstract,
            length: chrono::Duration::seconds(result.duration as i64),
            thumbnail_path: result.thumb_url,
            uploaded_at: chrono::Utc.timestamp(uploaded_time, 0),
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
