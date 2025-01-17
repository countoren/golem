use std::sync::Arc;

use golem_worker_service_base::api::ApiEndpointError;
use golem_worker_service_base::api_definition::{ApiDefinitionId, ApiSiteString};
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::*;

use golem_service_base::api_tags::ApiTags;
use golem_worker_service_base::auth::CommonNamespace;
use tracing::log::info;

use golem_worker_service_base::api::ApiDeployment;
use golem_worker_service_base::api_definition;
use golem_worker_service_base::service::api_definition::ApiDefinitionKey;
use golem_worker_service_base::service::api_deployment::ApiDeploymentService;

pub struct ApiDeploymentApi {
    deployment_service: Arc<dyn ApiDeploymentService<CommonNamespace> + Sync + Send>,
}

#[OpenApi(prefix_path = "/v1/api/deployments", tag = ApiTags::ApiDeployment)]
impl ApiDeploymentApi {
    pub fn new(
        deployment_service: Arc<dyn ApiDeploymentService<CommonNamespace> + Sync + Send>,
    ) -> Self {
        Self { deployment_service }
    }

    #[oai(path = "/deploy", method = "post", operation_id = "deploy")]
    async fn create_or_update(
        &self,
        payload: Json<ApiDeployment>,
    ) -> Result<Json<ApiDeployment>, ApiEndpointError> {
        info!(
            "Deploy API definition - id: {}, version: {}, site: {}",
            payload.api_definition_id, payload.version, payload.site
        );

        let api_deployment = api_definition::ApiDeployment {
            api_definition_id: ApiDefinitionKey {
                namespace: CommonNamespace::default(),
                id: payload.api_definition_id.clone(),
                version: payload.version.clone(),
            },
            site: payload.site.clone(),
        };

        self.deployment_service.deploy(&api_deployment).await?;

        let data = self
            .deployment_service
            .get_by_host(&ApiSiteString::from(&payload.site))
            .await?;

        let deployment = data.ok_or(ApiEndpointError::internal(
            "Failed to verify the deployment",
        ))?;

        Ok(Json(deployment.into()))
    }

    #[oai(path = "/", method = "get", operation_id = "list_deployments")]
    async fn list(
        &self,
        #[oai(name = "api-definition-id")] api_definition_id_query: Query<ApiDefinitionId>,
    ) -> Result<Json<Vec<ApiDeployment>>, ApiEndpointError> {
        let api_definition_id = api_definition_id_query.0;

        info!("Get API deployments - id: {}", api_definition_id);

        let values = self
            .deployment_service
            .get_by_id(&CommonNamespace::default(), &api_definition_id)
            .await?;

        Ok(Json(values.iter().map(|v| v.clone().into()).collect()))
    }

    #[oai(path = "/:site", method = "get", operation_id = "get_deployment")]
    async fn get(&self, site: Path<String>) -> Result<Json<ApiDeployment>, ApiEndpointError> {
        let site = site.0;

        info!("Get API deployments for site: {site}");

        let value = self
            .deployment_service
            .get_by_host(&ApiSiteString(site))
            .await?
            .ok_or(ApiEndpointError::not_found("Api deployment not found"))?;

        Ok(Json(value.into()))
    }

    #[oai(path = "/:site", method = "delete", operation_id = "delete_deployment")]
    async fn delete(&self, site: Path<String>) -> Result<Json<String>, ApiEndpointError> {
        let site = site.0;

        self.deployment_service
            .delete(&CommonNamespace::default(), &ApiSiteString(site))
            .await?;

        Ok(Json("API deployment deleted".to_string()))
    }
}
