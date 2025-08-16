use clap::Parser;
use color_eyre::{Result, eyre::OptionExt};
use critique_api::MediaUniverse;
use rand::seq::IteratorRandom;

#[derive(Debug, Parser)]
pub struct CliArgs {
    /// SensCritique username
    pub username: String,
    /// Media type
    #[arg(short, long, default_value = "film", value_parser = parse_media_universe)]
    pub media_type: MediaUniverse,
    /// Number of items to randomly pick
    #[arg(short, long, default_value = "1")]
    pub count: usize,
}

fn parse_media_universe(s: &str) -> Result<MediaUniverse, String> {
    MediaUniverse::try_from(s).map_err(|_| {
        "Valid values: movie, book, game, tvShow, comicBook, musicAlbum, musicTrack".to_string()
    })
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = CliArgs::parse();
    let mut rng = rand::rng();
    let client = critique_api::CritiqueClient::new();

    println!("[*] Fetching user collection...");

    let res = client
        .get_user_collection(&args.username, None, None, Some(args.media_type))
        .await?;
    let collection = res
        .user
        .ok_or_eyre("no user in response")?
        .collection
        .ok_or_eyre("no collection in response")?
        .products
        .ok_or_eyre("no products in response")?;

    let picks = collection
        .into_iter()
        .filter_map(|m| {
            let m = m?;
            if m.other_user_infos.as_ref()?.is_wished {
                Some(m)
            } else {
                None
            }
        })
        .choose_multiple(&mut rng, args.count);

    for pick in picks {
        println!("[+] Random {}: {}", args.media_type.to_string(), pick.title);
        if let Some(year) = pick.year_of_production {
            println!("    Year: {}", year);
        }
        println!("    url: https://senscritique.com{}", pick.url);
    }

    Ok(())
}
