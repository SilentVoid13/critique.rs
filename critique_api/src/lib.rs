mod error;

use graphql_client::{GraphQLQuery, Response};
use reqwest::blocking::Client;

use error::Result;

use crate::error::CritiqueError;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.graphql",
    query_path = "gql/query/user/user_collection.graphql",
    response_derives = "Debug"
)]
pub struct UserCollectionQuery;

pub struct CritiqueClient {
    client: Client,
}

pub enum MediaUniverse {
    Film = 1,
    Book = 2,
    Videogame = 3,
    TvShow = 4,
    ComicBook = 5,
    MusicAlbum = 7,
    MusicTrack = 8,
}

impl ToString for MediaUniverse {
    fn to_string(&self) -> String {
        match self {
            MediaUniverse::Film => "movie".to_string(),
            MediaUniverse::Book => "book".to_string(),
            MediaUniverse::Videogame => "game".to_string(),
            MediaUniverse::TvShow => "tvShow".to_string(),
            MediaUniverse::ComicBook => "comicBook".to_string(),
            MediaUniverse::MusicAlbum => "musicAlbum".to_string(),
            MediaUniverse::MusicTrack => "musicTrack".to_string(),
        }
    }
}

impl CritiqueClient {
    const GQL_ENDPOINT: &'static str = "https://apollo.senscritique.com/";

    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn get_user_collection(&self, username: &str, limit: Option<i64>, offset: Option<i64>, universe: Option<MediaUniverse>) -> Result<user_collection_query::ResponseData> {
        let variables = user_collection_query::Variables {
            username: username.to_string(),
            limit,
            offset,
            universe: universe.map(|u| u.to_string()),
        };
        let req_body = UserCollectionQuery::build_query(variables);
        let res = self
            .client
            .post(Self::GQL_ENDPOINT)
            .json(&req_body)
            .send()?;
        let gql_res: Response<user_collection_query::ResponseData> = res.json()?;
        let user_collection = gql_res.data.ok_or(CritiqueError::NoDataInResponse)?;
        Ok(user_collection)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CritiqueClient, MediaUniverse};

    #[test]
    pub fn test_get_user_collection() {
        let client = CritiqueClient::new();
        let res = client.get_user_collection("Sergent_Pepper", Some(100), None, Some(MediaUniverse::ComicBook)).unwrap();
        dbg!(res);
    }
}
