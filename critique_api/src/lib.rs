mod error;

use graphql_client::{GraphQLQuery, Response};
use reqwest::{Client, header::HeaderMap};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::error::CritiqueError;
use error::Result;

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

#[derive(Debug, Clone, Copy, PartialEq, EnumString, EnumIter, Display)]
#[strum(serialize_all = "camelCase")]
pub enum MediaUniverse {
    #[strum(serialize = "movie")]
    Film = 1,
    Book = 2,
    #[strum(serialize = "game")]
    Videogame = 3,
    TvShow = 4,
    ComicBook = 5,
    MusicAlbum = 7,
    MusicTrack = 8,
}

pub fn valid_media_universes() -> Vec<String> {
    MediaUniverse::iter().map(|m| m.to_string()).collect()
}

impl CritiqueClient {
    const GQL_ENDPOINT: &'static str = "https://apollo.senscritique.com/";

    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("user-agent", "Mozilla/5.0".parse().unwrap());

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { client }
    }

    pub async fn get_user_collection(
        &self,
        username: &str,
        limit: Option<i64>,
        offset: Option<i64>,
        universe: Option<MediaUniverse>,
    ) -> Result<user_collection_query::ResponseData> {
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
            .send()
            .await?;

        let gql_res: Response<user_collection_query::ResponseData> = res.json().await?;
        let user_collection = gql_res.data.ok_or(CritiqueError::NoDataInResponse)?;
        Ok(user_collection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user_collection() {
        let client = CritiqueClient::new();
        let res = client
            .get_user_collection(
                "Sergent_Pepper",
                Some(100),
                None,
                Some(MediaUniverse::ComicBook),
            )
            .await
            .unwrap();
        dbg!(res);
    }
}
