use std::result::Result;

use reqwest::{Error, Response};
use serde::Serialize;
use url::Url;

use super::{
    BalancePayload, DepositAddressPayload, DepositPayload, RegisterFedPayload, WithdrawPayload,
};

pub struct RpcClient {
    // Base URL to gateway web server
    base_url: Url,
    // A request client
    client: reqwest::Client,
}

impl RpcClient {
    pub fn new(base_url: Url) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_info(&self, password: String) -> Result<Response, Error> {
        let url = self.base_url.join("/info").expect("invalid base url");
        self.call(url, password, ()).await
    }

    pub async fn get_balance(
        &self,
        password: String,
        payload: BalancePayload,
    ) -> Result<Response, Error> {
        let url = self.base_url.join("/balance").expect("invalid base url");
        self.call(url, password, payload).await
    }

    pub async fn get_deposit_address(
        &self,
        password: String,
        payload: DepositAddressPayload,
    ) -> Result<Response, Error> {
        let url = self.base_url.join("/address").expect("invalid base url");
        self.call(url, password, payload).await
    }

    pub async fn deposit(
        &self,
        password: String,
        payload: DepositPayload,
    ) -> Result<Response, Error> {
        let url = self.base_url.join("/deposit").expect("invalid base url");
        self.call(url, password, payload).await
    }

    pub async fn withdraw(
        &self,
        password: String,
        payload: WithdrawPayload,
    ) -> Result<Response, Error> {
        let url = self.base_url.join("/withdraw").expect("invalid base url");
        self.call(url, password, payload).await
    }

    pub async fn register_federation(
        &self,
        password: String,
        payload: RegisterFedPayload,
    ) -> Result<Response, Error> {
        let url = self.base_url.join("/register").expect("invalid base url");
        self.call(url, password, payload).await
    }

    async fn call<P>(
        &self,
        url: Url,
        password: String,
        payload: P,
    ) -> Result<Response, reqwest::Error>
    where
        P: Serialize,
    {
        self.client
            .post(url)
            .bearer_auth(password)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&payload)
            .send()
            .await
    }
}
