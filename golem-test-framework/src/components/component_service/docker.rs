// Copyright 2024 Golem Cloud
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use testcontainers::core::WaitFor;
use testcontainers::{Container, Image, RunnableImage};
use tonic::transport::Channel;
use tracing::{info, Level};

use golem_api_grpc::proto::golem::component::component_service_client::ComponentServiceClient;

use crate::components::component_service::{new_client, ComponentService, ComponentServiceEnvVars};
use crate::components::docker::KillContainer;
use crate::components::rdb::Rdb;
use crate::components::{GolemEnvVars, DOCKER, NETWORK};

pub struct DockerComponentService {
    container: Container<'static, GolemComponentServiceImage>,
    keep_container: bool,
    public_http_port: u16,
    public_grpc_port: u16,
    client: Option<ComponentServiceClient<Channel>>,
}

impl DockerComponentService {
    const NAME: &'static str = "golem_component_service";
    const HTTP_PORT: u16 = 8081;
    const GRPC_PORT: u16 = 9091;

    pub async fn new(
        component_compilation_service: Option<(&str, u16)>,
        rdb: Arc<dyn Rdb + Send + Sync + 'static>,
        verbosity: Level,
        shared_client: bool,
        keep_container: bool,
    ) -> Self {
        Self::new_base(
            Box::new(GolemEnvVars()),
            component_compilation_service,
            rdb,
            verbosity,
            shared_client,
            keep_container,
        )
        .await
    }

    pub async fn new_base(
        env_vars: Box<dyn ComponentServiceEnvVars + Send + Sync + 'static>,
        component_compilation_service: Option<(&str, u16)>,
        rdb: Arc<dyn Rdb + Send + Sync + 'static>,
        verbosity: Level,
        shared_client: bool,
        keep_container: bool,
    ) -> Self {
        info!("Starting golem-component-service container");

        let env_vars = env_vars
            .env_vars(
                Self::HTTP_PORT,
                Self::GRPC_PORT,
                component_compilation_service,
                rdb,
                verbosity,
            )
            .await;

        let image = RunnableImage::from(GolemComponentServiceImage::new(
            Self::GRPC_PORT,
            Self::HTTP_PORT,
            env_vars,
        ))
        .with_container_name(Self::NAME)
        .with_network(NETWORK);

        let container = DOCKER.run(image);

        let public_http_port = container.get_host_port_ipv4(Self::HTTP_PORT);
        let public_grpc_port = container.get_host_port_ipv4(Self::GRPC_PORT);

        Self {
            container,
            keep_container,
            public_http_port,
            public_grpc_port,
            client: if shared_client {
                Some(new_client("localhost", public_grpc_port).await)
            } else {
                None
            },
        }
    }
}

#[async_trait]
impl ComponentService for DockerComponentService {
    async fn client(&self) -> ComponentServiceClient<Channel> {
        match &self.client {
            Some(client) => client.clone(),
            None => new_client("localhost", self.public_grpc_port).await,
        }
    }

    fn private_host(&self) -> String {
        Self::NAME.to_string()
    }

    fn private_http_port(&self) -> u16 {
        Self::HTTP_PORT
    }

    fn private_grpc_port(&self) -> u16 {
        Self::GRPC_PORT
    }

    fn public_host(&self) -> String {
        "localhost".to_string()
    }

    fn public_http_port(&self) -> u16 {
        self.public_http_port
    }

    fn public_grpc_port(&self) -> u16 {
        self.public_grpc_port
    }

    fn kill(&self) {
        self.container.kill(self.keep_container);
    }
}

impl Drop for DockerComponentService {
    fn drop(&mut self) {
        self.kill();
    }
}

#[derive(Debug)]
struct GolemComponentServiceImage {
    env_vars: HashMap<String, String>,
    expose_ports: [u16; 2],
}

impl GolemComponentServiceImage {
    pub fn new(
        grpc_port: u16,
        http_port: u16,
        env_vars: HashMap<String, String>,
    ) -> GolemComponentServiceImage {
        GolemComponentServiceImage {
            env_vars,
            expose_ports: [grpc_port, http_port],
        }
    }
}

impl Image for GolemComponentServiceImage {
    type Args = ();

    fn name(&self) -> String {
        "golemservices/golem-component-service".to_string()
    }

    fn tag(&self) -> String {
        "latest".to_string()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout("server started")]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }

    fn expose_ports(&self) -> Vec<u16> {
        self.expose_ports.to_vec()
    }
}
