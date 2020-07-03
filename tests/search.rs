extern crate spotify_api;

#[cfg(test)]
mod search {
    use spotify_api::search::*;

    #[tokio::test]
    async fn search() {
        dotenv::dotenv().ok();

        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();

        let mut client = SearchClient::new(&access_token, &refresh_token);

        let result = client
            .set_keyword("tania")
            .set_keyword("bowra")
            .search_artist()
            .await
            .unwrap();

        dbg!(result);
    }
}
