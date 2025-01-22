use twitter_v2::{authorization::Oauth1aToken, TwitterApi, id::IntoNumericId};
use reqwest::multipart;
use serde::Deserialize;
use reqwest_oauth1::OAuthClientProvider;
#[derive(Debug, Deserialize)]
struct MediaUploadResponse {
    media_id: u64,
}
pub struct Twitter {
    auth: Oauth1aToken,
    twitter_consumer_key: String,
    twitter_consumer_secret: String,
    twitter_access_token: String,
    twitter_access_token_secret: String,
}

impl Twitter {
    pub fn new(
        twitter_consumer_key: &str,
        twitter_consumer_secret: &str,
        twitter_access_token: &str,
        twitter_access_token_secret: &str,
    ) -> Self {
        let auth = Oauth1aToken::new(
            twitter_consumer_key.to_string(),
            twitter_consumer_secret.to_string(),
            twitter_access_token.to_string(),
            twitter_access_token_secret.to_string(),
        );
        Twitter {
            auth,
            twitter_consumer_key: twitter_consumer_key.to_string(),
            twitter_consumer_secret: twitter_consumer_secret.to_string(),
            twitter_access_token: twitter_access_token.to_string(),
            twitter_access_token_secret: twitter_access_token_secret.to_string(),
        }
    }

    pub async fn tweet_with_image(&self, text: String, media_id: u64, user_id: impl IntoNumericId) -> Result<(), anyhow::Error> {
        let tweet = TwitterApi::new(self.auth.clone())
            .post_tweet()
            .add_media([media_id], [user_id])
            .text(text)
            .send()
            .await?
            .into_data()
            .expect("this tweet should exist");
        println!("Tweet posted successfully with ID: {}", tweet.id);

        Ok(())
    }
    pub async fn tweet(&self, text: String) -> Result<(), anyhow::Error> {
        let tweet = TwitterApi::new(self.auth.clone())
            .post_tweet()
            .text(text)
            .send()
            .await?
            .into_data()
            .expect("this tweet should exist");
        println!("Tweet posted successfully with ID: {}", tweet.id);

        Ok(())
    }

    pub async fn reply_to_tweet(&self, tweet_id: &str, text: String) -> Result<(), anyhow::Error> {
        let tweet_id = tweet_id.parse::<u64>()?;
        let tweet = TwitterApi::new(self.auth.clone())
            .post_tweet()
            .in_reply_to_tweet_id(tweet_id)
            .text(text)
            .send()
            .await?
            .into_data()
            .expect("this tweet should exist");
        println!("Reply posted successfully with ID: {}", tweet.id);

        Ok(())
    }
    
    pub async fn get_notifications(&self, user_id: impl IntoNumericId) -> Result<Vec<twitter_v2::Tweet>, anyhow::Error> {
        let api = TwitterApi::new(self.auth.clone());
        let mentions = api
            .get_user_mentions(user_id)
            .send()
            .await?
            .into_data()
            .unwrap_or_default();

        Ok(mentions)
    }

    pub async fn get_user_id(&self) -> Result<impl IntoNumericId, anyhow::Error> {
        let api = TwitterApi::new(self.auth.clone());
        let me = api.get_users_me()
            .send()
            .await?
            .into_data()
            .expect("should have user data");
        
        Ok(me.id)
    }
    
    pub async fn upload_bytes(&self, bytes: Vec<u8>) -> Result<u64, anyhow::Error> {
        let part = multipart::Part::bytes(bytes);

        let form = multipart::Form::new().part("media", part);

        // Extract OAuth credentials from the auth token
        let secrets = reqwest_oauth1::Secrets::new(&self.twitter_consumer_key, &self.twitter_consumer_secret)
            .token(&self.twitter_access_token, &self.twitter_access_token_secret);

        let client = reqwest::Client::new();
        let response = client
            .oauth1(secrets)
            .post("https://upload.twitter.com/1.1/media/upload.json")
            .multipart(form)
            .send()
            .await;
        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let media_response = res.json::<MediaUploadResponse>().await?;
                    Ok(media_response.media_id)
                } else {
                    Err(anyhow::anyhow!("Failed to upload media: {}", res.status()))
                }
            }
            Err(err) => Err(anyhow::anyhow!("Failed to upload media: {}", err))
        }
    }
}
