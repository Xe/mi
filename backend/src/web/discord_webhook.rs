use super::{Error, Result};
use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Debug, Deserialize)]
struct AllowedMentions {
    parse: Vec<String>,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct Body {
    content: String,
    allowed_mentions: AllowedMentions,
}

impl Body {
    pub fn new<T>(body: T) -> Body
    where
        T: Into<String>,
    {
        Body {
            content: body.into(),
            allowed_mentions: AllowedMentions { parse: vec![] },
        }
    }
}

pub struct Client {
    webhook_url: String,
}

impl Client {
    pub fn new(webhook_url: String) -> Self {
        Client {
            webhook_url: webhook_url,
        }
    }

    pub fn fairing() -> AdHoc {
        AdHoc::on_attach("Switch Counter API", |rocket| {
            let webhook_url = rocket.config().get_string("discord_webhook").unwrap();
            Ok(rocket.manage(Client::new(webhook_url)))
        })
    }

    #[instrument(skip(self), err)]
    pub fn send(&self, body: String) -> Result<()> {
        ureq::post(&self.webhook_url)
            .set("User-Agent", crate::APPLICATION_NAME)
            .send_json(serde_json::to_value(Body::new(body))?)
            .map_err(Error::UReq)?;

        Ok(())
    }
}
