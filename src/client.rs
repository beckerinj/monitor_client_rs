use std::collections::HashMap;

use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};

use crate::types::{CreateDeploymentBody, Deployment, LoginCredentials};

#[derive(Clone, Debug)]
pub struct Client {
    url: String,
    token: String,
    http_client: reqwest::Client,
}

impl Client {
    pub async fn new(url: &str, username: &str, password: &str) -> Client {
        let http_client = reqwest::Client::new();
        let url = Client::parse_url(url);
        Client {
            url: url.to_string(),
            token: Client::login(&http_client, url, username, password).await,
            http_client,
        }
    }

    pub fn new_with_token(url: &str, token: &str) -> Client {
        let http_client = reqwest::Client::new();
        let url = Client::parse_url(url).to_string();
        Client {
            url,
            token: token.to_string(),
            http_client,
        }
    }

    fn parse_url(url: &str) -> &str {
        if url.chars().nth(url.len() - 1).unwrap() == '/' {
            &url[..url.len() - 1]
        } else {
            url
        }
    }

    pub async fn create_deployment(
        &self,
        deployment: Deployment,
    ) -> Result<Deployment, String> {
        self.post::<CreateDeploymentBody, Deployment>(
            "/api/deployment/create",
            deployment.into_create_body(),
        )
        .await
    }

    pub async fn deploy(&self, deployment_id: &str) -> Result<String, String> {
        self.get_string(&format!("/api/deployment/{deployment_id}/deploy"))
            .await
    }

    pub async fn get_deployment(&self, deployment_id: &str) -> Result<Deployment, String> {
        self.get(&format!("/api/deployment/{deployment_id}")).await
    }

    pub async fn delete_deployment(&self, deployment_id: &str) -> Result<String, String> {
        self.delete_string(&format!("/api/deployment/{deployment_id}/delete"))
            .await
    }

    pub async fn get_deployments(&self) -> Result<HashMap<String, Deployment>, String> {
        self.get("/api/deployments").await
    }

    pub async fn delete_all_deployments_on_server<Callback>(
        &self,
        server_id: &str,
        on_delete: impl Into<Option<Callback>>,
    ) -> Result<(), String>
    where
        Callback: Fn(Deployment) -> (),
    {
        let deployments: Vec<(String, Deployment)> = self
            .get_deployments()
            .await?
            .into_iter()
            .filter(|(_, d)| d.server_id == server_id)
            .collect();

        if let Some(on_delete) = on_delete.into() {
            for (id, deployment) in deployments {
                self.delete_deployment(&id).await?;
                on_delete(deployment);
            }
        } else {
            for (id, _) in deployments {
                self.delete_deployment(&id).await?;
            }
        }

        Ok(())
    }

    async fn login(client: &reqwest::Client, url: &str, username: &str, password: &str) -> String {
        client
            .post(format!("{url}/login/local"))
            .json(&LoginCredentials::new(username, password))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    async fn get<R: DeserializeOwned>(&self, endpoint: &str) -> Result<R, String> {
        let res = self.http_client
            .get(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await;
        match res {
            Ok(res) => {
                let status = res.status();
                if status == StatusCode::OK {
                    match res.json().await {
                        Ok(res) => Ok(res),
                        Err(e) => Err(format!("{status}: {e:#?}")),
                    }
                } else {
                    match res.text().await {
                        Ok(res) => Err(format!("{status}: {res}")),
                        Err(e) => Err(format!("{status}: {e:#?}"))
                    }
                }
            }
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    async fn get_string(&self, endpoint: &str) -> Result<String, String> {
        let res = self
            .http_client
            .get(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await;

        match res {
            Ok(res) => {
                let status = res.status();
                if status == StatusCode::OK {
                    match res.text().await {
                        Ok(res) => Ok(res),
                        Err(e) => Err(format!("{status}: {e:#?}")),
                    }
                } else {
                    match res.text().await {
                        Ok(res) => Err(format!("{status}: {res}")),
                        Err(e) => Err(format!("{status}: {e:#?}"))
                    }
                }
            }
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    async fn post<B: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: B,
    ) -> Result<R, String> {
        let res = self.http_client
            .post(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;
        
        match res {
            Ok(res) => {
                let status = res.status();
                if status == StatusCode::OK {
                    match res.json().await {
                        Ok(res) => Ok(res),
                        Err(e) => Err(format!("{status}: {e:#?}")),
                    }
                } else {
                    match res.text().await {
                        Ok(res) => Err(format!("{status}: {res}")),
                        Err(e) => Err(format!("{status}: {e:#?}"))
                    }
                }
            }
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    async fn delete<R: DeserializeOwned>(&self, endpoint: &str) -> Result<R, String> {
        let res = self.http_client
            .delete(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await;

        match res {
            Ok(res) => {
                let status = res.status();
                if status == StatusCode::OK {
                    match res.json().await {
                        Ok(res) => Ok(res),
                        Err(e) => Err(format!("{status}: {e:#?}")),
                    }
                } else {
                    match res.text().await {
                        Ok(res) => Err(format!("{status}: {res}")),
                        Err(e) => Err(format!("{status}: {e:#?}"))
                    }
                }
            }
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    async fn delete_string(&self, endpoint: &str) -> Result<String, String> {
        let res = self.http_client
            .delete(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await;

        match res {
            Ok(res) => {
                let status = res.status();
                if status == StatusCode::OK {
                    match res.text().await {
                        Ok(res) => Ok(res),
                        Err(e) => Err(format!("{status}: {e:#?}")),
                    }
                } else {
                    match res.text().await {
                        Ok(res) => Err(format!("{status}: {res}")),
                        Err(e) => Err(format!("{status}: {e:#?}"))
                    }
                }
            }
            Err(e) => Err(format!("{e:#?}")),
        }
    }
}
