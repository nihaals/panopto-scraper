mod api;
mod custom_types;
mod get_sessions_request;
mod get_sessions_response;

#[tokio::main]
async fn main() {
    let host = std::env::var("HOST").expect("HOST not set");
    let cookie = std::env::var("COOKIE").expect("COOKIE not set");
    let folder_id = std::env::var("FOLDER_ID").expect("FOLDER_ID not set");

    let folder = api::Client::new(host, &cookie)
        .get_folder_from_id(folder_id)
        .await
        .unwrap();
    println!("{:?}", folder);
}
