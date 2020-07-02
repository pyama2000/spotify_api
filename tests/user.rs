extern crate spotify_api;

#[cfg(test)]
mod user {
    use spotify_api::user::*;

    #[tokio::test]
    async fn get_current_user() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = UserClient::new(&access_token, &refresh_token);
        client.get_current_user().await.unwrap();
    }

    #[tokio::test]
    async fn get_user() {
        dotenv::dotenv().ok();
    
        let access_token = std::env::var("ACCESS_TOKEN").unwrap();
        let refresh_token = std::env::var("REFRESH_TOKEN").unwrap();
    
        let mut client = UserClient::new(&access_token, &refresh_token);
        let user = client.get_user("tuggareutangranser").await.unwrap();

        assert_eq!("Lilla Namo", &user.display_name.unwrap());
    }
}
