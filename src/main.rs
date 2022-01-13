mod api;
mod custom_types;

mod delivery_info_request;
mod delivery_info_response;
mod get_sessions_request;
mod get_sessions_response;

#[tokio::main]
async fn main() {
    let host = std::env::var("HOST").expect("HOST not set");
    let cookie = std::env::var("COOKIE").expect("COOKIE not set");
    let folder_id = std::env::var("FOLDER_ID").expect("FOLDER_ID not set");

    let client = api::Client::new(host, &cookie);
    let folder = client.get_folder_from_id(folder_id).await.unwrap();
    let video = &folder.videos()[0];
    println!("{:?}", client.get_stream_info(video).await.unwrap());
}
