use std::sync::atomic::{AtomicU64, Ordering};

use crate::Error;

const DEFAULT_API_URL: &str = "https://api.guildwars2.com";
const BACKOFF_BASE: u64 = 300;
static INCREMENTAL_BACKOFF: AtomicU64 = AtomicU64::new(BACKOFF_BASE);

#[derive(Debug, Clone)]
pub struct Gw2Client {
    client: reqwest::Client,
    url_base: String,
    api_key: String,
}

impl Default for Gw2Client {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            url_base: DEFAULT_API_URL.to_string(),
            api_key: String::new(),
        }
    }
}

impl From<reqwest::Client> for Gw2Client {
    fn from(client: reqwest::Client) -> Self {
        Self {
            client,
            url_base: DEFAULT_API_URL.to_string(),
            api_key: String::new(),
        }
    }
}

impl Gw2Client {
    pub fn new(client: reqwest::Client, url_base: impl ToString, api_key: impl ToString) -> Self {
        Self {
            client,
            url_base: url_base.to_string(),
            api_key: api_key.to_string(),
        }
    }
    pub fn url_base(mut self, base: impl ToString) -> Self {
        self.with_url_base(base);
        self
    }

    pub fn with_url_base(&mut self, base: impl ToString) {
        self.url_base = base.to_string();
    }

    pub fn api_key(mut self, key: impl ToString) -> Self {
        self.with_api_key(key);
        self
    }

    pub fn with_api_key(&mut self, key: impl ToString) {
        self.api_key = key.to_string();
    }

    pub async fn get_all_currencies(&self) -> Result<Vec<crate::codec::Currency>, Error> {
        crate::endpoints::get_all_currencies(&self.client, &self.url_base, &self.api_key).await
    }
    pub async fn get_account_wallet(&self) -> Result<Vec<crate::codec::account::Wallet>, Error> {
        crate::endpoints::account::wallet(&self.client, &self.url_base, &self.api_key).await
    }
    pub async fn get_all_characters(
        &self,
    ) -> Result<Vec<crate::codec::characters::Character>, Error> {
        crate::endpoints::characters::get_all_characters(
            &self.client,
            &self.url_base,
            &self.api_key,
        )
        .await
    }
    pub async fn get_character(
        &self,
        name: &str,
    ) -> Result<crate::codec::characters::Character, Error> {
        crate::endpoints::characters::character(&self.client, &self.url_base, &self.api_key, name)
            .await
    }

    pub async fn item(&self, id: u64) -> Result<crate::codec::Item, Error> {
        let mut err = Error::Unknown;
        for _i in 0..5 {
            match crate::endpoints::item(&self.client, &self.url_base, &self.api_key, id).await {
                Ok(value) => {
                    INCREMENTAL_BACKOFF.store(BACKOFF_BASE, Ordering::Relaxed);
                    return Ok(value);
                }
                Err(Error::RateLimit) => {
                    let backoff = INCREMENTAL_BACKOFF.load(Ordering::Relaxed);
                    INCREMENTAL_BACKOFF.store(backoff << 1, Ordering::Relaxed);
                    tokio::time::sleep(std::time::Duration::from_millis(backoff)).await;
                    err = Error::RateLimit;
                }
                Err(e) => {
                    err = e.into();
                }
            }
        }
        Err(err)
    }

    pub async fn get_luck(&self) -> Result<crate::codec::account::Luck, Error> {
        let list =
            crate::endpoints::account::luck(&self.client, &self.url_base, &self.api_key).await?;
        let mut ret = crate::codec::account::Luck { value: 0 };
        for luck in list {
            ret.value += luck.value
        }
        Ok(ret)
    }

    pub async fn get_acct_mats(&self) -> Result<Vec<crate::codec::account::Material>, Error> {
        crate::endpoints::account::materials(&self.client, &self.url_base, &self.api_key).await
    }

    pub async fn get_all_material_defs(&self) -> Result<Vec<crate::codec::Item>, Error> {
        let mut ret = Vec::new();
        let cats =
            crate::endpoints::all_materials(&self.client, &self.url_base, &self.api_key).await?;
        for cat in cats {
            for chunk in cat.items.chunks(75) {
                ret.extend(
                    crate::endpoints::items_by_ids(
                        &self.client,
                        &self.url_base,
                        &self.api_key,
                        chunk.iter(),
                    )
                    .await?,
                );
            }
        }

        Ok(ret)
    }
}
