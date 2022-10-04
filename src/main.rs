#![warn(clippy::cast_lossless)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::default_trait_access)]
#![warn(clippy::else_if_without_else)]
#![warn(clippy::empty_enum)]
#![warn(clippy::empty_line_after_outer_attr)]
#![warn(clippy::enum_glob_use)]
#![warn(clippy::equatable_if_let)]
#![warn(clippy::float_cmp)]
#![warn(clippy::fn_params_excessive_bools)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::inefficient_to_string)]
#![warn(clippy::integer_division)]
#![warn(clippy::let_unit_value)]
#![warn(clippy::linkedlist)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::manual_assert)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::many_single_char_names)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::match_bool)]
#![warn(clippy::match_on_vec_items)]
#![warn(clippy::match_same_arms)]
#![warn(clippy::match_wild_err_arm)]
#![warn(clippy::match_wildcard_for_single_variants)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::must_use_candidate)]
#![warn(clippy::mut_mut)]
#![warn(clippy::negative_feature_names)]
#![warn(non_ascii_idents)]
#![warn(clippy::option_option)]
#![warn(clippy::redundant_feature_names)]
#![warn(clippy::redundant_pub_crate)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
#![warn(clippy::trait_duplication_in_bounds)]
#![warn(clippy::unused_async)]
#![warn(clippy::unused_self)]
#![warn(clippy::use_self)]
#![warn(clippy::wildcard_dependencies)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::zero_sized_map_values)]

mod api;
mod custom_types;

mod raw_types;

use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List a folder's videos
    List {
        /// The folder's id
        folder_id: String,

        /// The `.ASPXAUTH` cookie to use for authentication
        #[arg(short, long)]
        cookie: String,

        /// The host to use
        #[arg(
            short = 'H',
            long,
            long_help = "The host to use\n\
            Example: 'example.cloud.panopto.eu'\n\
            Note: Always uses HTTPS and should not have a trailing slash"
        )]
        host: String,

        /// Fetch recursively
        #[arg(short = 'R', long)]
        recursive: bool,

        /// Fetch additional streams
        #[arg(short = 'S', long)]
        fetch_streams: bool,

        /// Include folders
        #[arg(short = 'F', long)]
        include_folders: bool,
    },

    /// Fetch a specific video
    Fetch {
        /// The video's id
        video_id: String,

        /// The `.ASPXAUTH` cookie to use for authentication
        #[arg(short, long)]
        cookie: String,

        /// The host to use
        #[arg(
            short = 'H',
            long,
            long_help = "The host to use\n\
            Example: 'example.cloud.panopto.eu'\n\
            Note: Always uses HTTPS and should not have a trailing slash"
        )]
        host: String,
    },

    /// Generate shell completions
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
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
            println!("{:#?}", videos);
        }

        Commands::Fetch {
            cookie,
            video_id,
            host,
        } => {
            let client = api::Client::new(host, &cookie);
            let video = client.get_video_from_id(video_id).await.unwrap();
            println!("{:#?}", video);
        }
    }
}
