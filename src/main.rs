mod api;
mod custom_types;

mod delivery_info_request;
mod delivery_info_response;
mod get_sessions_request;
mod get_sessions_response;

use clap::{ErrorKind, IntoApp, Parser, Subcommand};

#[derive(Parser)]
#[clap(version, author, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List a folder's videos
    List {
        /// The folder's id
        folder_id: String,

        /// The `.ASPXAUTH` cookie to use for authentication
        #[clap(short, long)]
        cookie: String,

        /// The host to use
        #[clap(
            short = 'H',
            long,
            long_help = "The host to use\n\
            Example: 'example.cloud.panopto.eu'\n\
            Note: Always uses HTTPS and should not have a trailing slash"
        )]
        host: String,

        /// Fetch recursively
        #[clap(short = 'R', long)]
        recursive: bool,

        /// Fetch additional streams
        #[clap(short = 'S', long)]
        fetch_streams: bool,

        /// Include folders
        #[clap(short = 'F', long)]
        include_folders: bool,
    },

    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[clap(arg_enum)]
        shell: clap_complete_command::Shell,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Completions { shell } => {
            shell.generate(&mut Cli::command(), &mut std::io::stdout());
        }
        Commands::List {
            cookie,
            fetch_streams,
            folder_id,
            host,
            include_folders,
            recursive,
        } => {
            let client = api::Client::new(host, &cookie);
            let folder = client.get_folder_from_id(folder_id).await.unwrap();
            let mut videos = if !recursive {
                folder.videos().to_vec()
            } else {
                if include_folders {
                    let mut app = Cli::command();
                    app.error(
                        ErrorKind::ArgumentConflict,
                        "Can't include folders when fetching recursively",
                    )
                    .exit();
                }

                let mut videos = folder.videos().to_vec();
                let mut folders = folder.folders().to_vec();
                while !folders.is_empty() {
                    let next_folder = folders.pop().unwrap();
                    let folder_videos = client.get_folder_listing(&next_folder).await.unwrap();
                    videos.extend(folder_videos.videos().iter().cloned());
                    folders.extend(folder_videos.folders().iter().cloned());
                }
                videos
            };

            if fetch_streams {
                for video in videos.iter_mut() {
                    client.get_streams(video).await.unwrap();
                }
            }
            println!("{:?}", videos);
        }
    }
}
