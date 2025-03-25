mod args;

use clap::Parser;
use color_eyre::{eyre::OptionExt, Result};
use critique_api::MediaUniverse;

#[derive(serde::Serialize)]
struct Record {
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Rating10")]
    rating: i64,
    #[serde(rename = "Year")]
    year: Option<i64>,
    #[serde(rename = "WatchedDate")]
    watch_date: Option<String>,
    #[serde(rename = "Review")]
    review: Option<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = args::CliArgs::parse();
    let mut writer = csv::Writer::from_path(args.output.clone())?;
    let client = critique_api::CritiqueClient::new();

    println!("[*] Fetching user collection...");

    let media_universe;
    if args.media_type == "film" {
        media_universe = MediaUniverse::Film;
    } else if args.media_type == "tvShow" {
        media_universe = MediaUniverse::TvShow;
    } else if args.media_type == "game" {
        media_universe = MediaUniverse::Videogame;
    } else if args.media_type == "book" {
        media_universe = MediaUniverse::Book;
    } else if args.media_type == "comicBook" {
        media_universe = MediaUniverse::ComicBook;
    } else if args.media_type == "musicAlbum" {
        media_universe = MediaUniverse::MusicAlbum;
    } else if args.media_type == "musicTrack" {
        media_universe = MediaUniverse::MusicTrack;
    } else {
        eprintln!("[-] Unsupported media type: {}", args.media_type);
        std::process::exit(1);
    }

    let res = client.get_user_collection(&args.username, None, None, Some(media_universe))?;
    let film_collection = res
        .user
        .ok_or_eyre("no user in response")?
        .collection
        .ok_or_eyre("no collection in response")?
        .products
        .ok_or_eyre("no products in response")?;

    // Only export films with a rating
    for product in film_collection.into_iter().filter_map(|p| p).filter(|p| {
        if let Some(info) = p.other_user_infos.as_ref() {
            if info.rating.is_some() {
                return true;
            }
        }
        false
    }) {
        let title = if let Some(t) = product.original_title {
            t
        } else {
            product.title
        };
        // SAFETY: we just checked that it is present
        let user_info = product.other_user_infos.unwrap();
        let rating = user_info.rating.unwrap();
        let watch_date = user_info.date_done;
        let review = if args.with_reviews {
            user_info.review.map(|r| r.body_text).flatten()
        } else {
            None
        };

        let record = Record {
            title,
            year: product.year_of_production,
            rating,
            watch_date,
            review,
        };
        writer.serialize(record)?;
    }

    println!(
        "[+] Sucessfully exported to {}",
        args.output.to_string_lossy()
    );
    println!("[*] You can now navigate to https://letterboxd.com/import/ to import this file");

    Ok(())
}
