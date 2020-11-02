use color_eyre::eyre::{eyre, Result};
use rocket::fairing::AdHoc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct FrontAsk {
    pub command: String,
}

impl Default for FrontAsk {
    fn default() -> Self {
        FrontAsk {
            command: "switch".to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct SwitchCommand {
    pub command: String,
    pub member_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub member_name: String,
    pub started_at: chrono::NaiveDateTime,
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
            let webhook_url = rocket.config().get_string("switchcounter_webhook").unwrap();
            Ok(rocket.manage(Client::new(webhook_url)))
        })
    }

    #[instrument(err, skip(self))]
    pub fn status(&self) -> Result<Status> {
        let resp =
            ureq::post(&self.webhook_url).send_json(serde_json::to_value(FrontAsk::default())?);
        if resp.ok() {
            Ok(resp.into_json_deserialize()?)
        } else {
            Err(eyre!("{}", resp.status_line()))
        }
    }

    #[instrument(err, skip(self))]
    pub fn switch(&self, member_name: String) -> Result<Status> {
        let resp = ureq::post(&self.webhook_url).send_json(serde_json::to_value(SwitchCommand {
            command: "switch".to_string(),
            member_name: member_name,
        })?);

        if resp.ok() {
            Ok(resp.into_json_deserialize()?)
        } else {
            Err(eyre!("{}", resp.status_line()))
        }
    }
}
