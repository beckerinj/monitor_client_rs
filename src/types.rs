use mungos::ObjectId;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::helpers::enum_as_string;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    #[serde(rename = "serverID")]
    pub server_id: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "buildID")]
    pub build_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<Conversion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Conversion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<EnvironmentVar>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_account: Option<String>,
}

impl Deployment {
    pub fn builder() -> DeploymentBuilder {
        DeploymentBuilder::new()
    }

    pub fn into_create_body(self) -> CreateDeploymentBody {
        CreateDeploymentBody { deployment: self }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateDeploymentBody {
    deployment: Deployment,
}

#[derive(Default)]
pub struct DeploymentBuilder {
    deployment: Deployment,
}

impl DeploymentBuilder {
    pub fn new() -> DeploymentBuilder {
        DeploymentBuilder {
            ..Default::default()
        }
    }
    pub fn name(mut self, name: &str) -> DeploymentBuilder {
        self.deployment.name = name.to_string();
        self
    }
    pub fn server_id(mut self, server_id: &str) -> DeploymentBuilder {
        self.deployment.server_id = server_id.to_string();
        self
    }
    pub fn build_id(mut self, build_id: impl Into<Option<String>>) -> DeploymentBuilder {
        self.deployment.build_id = build_id.into();
        self
    }
    pub fn image(mut self, image: impl Into<Option<String>>) -> DeploymentBuilder {
        self.deployment.image = image.into();
        self
    }
    pub fn docker_account(
        mut self,
        docker_account: impl Into<Option<String>>,
    ) -> DeploymentBuilder {
        self.deployment.docker_account = docker_account.into();
        self
    }
    pub fn add_environment(
        mut self,
        variable: impl Into<String>,
        value: impl Into<String>,
    ) -> DeploymentBuilder {
        let env_var = EnvironmentVar {
            variable: variable.into(),
            value: value.into(),
        };
        if let Some(env) = &mut self.deployment.environment {
            env.push(env_var);
        } else {
            let mut env: Vec<EnvironmentVar> = Vec::new();
            env.push(env_var);
            self.deployment.environment = Some(env);
        };
        self
    }
    pub fn add_port(
        mut self,
        local: impl Into<String>,
        container: impl Into<String>,
    ) -> DeploymentBuilder {
        let port = Conversion {
            local: local.into(),
            container: container.into(),
        };
        if let Some(ports) = &mut self.deployment.ports {
            ports.push(port);
        } else {
            let mut ports: Vec<Conversion> = Vec::new();
            ports.push(port);
            self.deployment.ports = Some(ports);
        };
        self
    }
    pub fn add_volume(
        mut self,
        local: impl Into<String>,
        container: impl Into<String>,
    ) -> DeploymentBuilder {
        let volume = Conversion {
            local: local.into(),
            container: container.into(),
        };
        if let Some(volumes) = &mut self.deployment.volumes {
            volumes.push(volume);
        } else {
            let mut volumes: Vec<Conversion> = Vec::new();
            volumes.push(volume);
            self.deployment.volumes = Some(volumes);
        };
        self
    }
    pub fn restart(mut self, restart: RestartMode) -> DeploymentBuilder {
        self.deployment.restart = Some(enum_as_string(&restart));
        self
    }
    pub fn network(mut self, network: &str) -> DeploymentBuilder {
        self.deployment.network = Some(network.to_string());
        self
    }
    pub fn build(self) -> Deployment {
        self.deployment
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversion {
    pub local: String,
    pub container: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvironmentVar {
    pub variable: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginCredentials {
    username: String,
    password: String,
}

impl LoginCredentials {
    pub fn new(username: &str, password: &str) -> LoginCredentials {
        LoginCredentials {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq)]
pub enum RestartMode {
    #[serde(rename = "no")]
    NoRestart,
    #[serde(rename = "unless-stopped")]
    UnlessStopped,
    #[serde(rename = "on-failure")]
    OnFailure,
    #[serde(rename = "always")]
    Always,
}
