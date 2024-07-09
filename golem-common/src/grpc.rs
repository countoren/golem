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

use golem_api_grpc::proto::golem::apidefinition;
use golem_api_grpc::proto::golem::common;
use golem_api_grpc::proto::golem::component;
use golem_api_grpc::proto::golem::worker;

use crate::model::{AccountId, ComponentId, IdempotencyKey, PromiseId, WorkerId};

pub fn proto_component_id_string(component_id: &Option<component::ComponentId>) -> Option<String> {
    component_id
        .clone()
        .and_then(|v| TryInto::<ComponentId>::try_into(v).ok())
        .map(|v| v.to_string())
}

pub fn proto_worker_id_string(worker_id: &Option<worker::WorkerId>) -> Option<String> {
    worker_id
        .clone()
        .and_then(|v| TryInto::<WorkerId>::try_into(v).ok())
        .map(|v| v.to_string())
}

pub fn proto_idempotency_key_string(
    idempotency_key: &Option<worker::IdempotencyKey>,
) -> Option<String> {
    idempotency_key
        .clone()
        .map(|v| Into::<IdempotencyKey>::into(v).to_string())
}

pub fn proto_account_id_string(account_id: &Option<common::AccountId>) -> Option<String> {
    account_id
        .clone()
        .map(|v| Into::<AccountId>::into(v).to_string())
}

pub fn proto_promise_id_string(promise_id: &Option<worker::PromiseId>) -> Option<String> {
    promise_id
        .clone()
        .and_then(|v| TryInto::<PromiseId>::try_into(v).ok())
        .map(|v| v.to_string())
}

pub fn proto_invocation_context_parent_worker_id_string(
    invocation_context: &Option<worker::InvocationContext>,
) -> Option<String> {
    proto_worker_id_string(&invocation_context.as_ref().and_then(|c| c.parent.clone()))
}

pub enum ProtoApiDefinitionKind {
    Golem,
    OpenAPI,
}

pub trait HasProtoApiDefinition {
    fn proto_api_definition_kind(&self) -> ProtoApiDefinitionKind;
    fn api_definition(&self) -> Option<&apidefinition::ApiDefinition>;
}

impl HasProtoApiDefinition for apidefinition::create_api_definition_request::ApiDefinition {
    fn proto_api_definition_kind(&self) -> ProtoApiDefinitionKind {
        match self {
            apidefinition::create_api_definition_request::ApiDefinition::Definition(_) => {
                ProtoApiDefinitionKind::Golem
            }
            apidefinition::create_api_definition_request::ApiDefinition::Openapi(_) => {
                ProtoApiDefinitionKind::OpenAPI
            }
        }
    }

    fn api_definition(&self) -> Option<&apidefinition::ApiDefinition> {
        match self {
            apidefinition::create_api_definition_request::ApiDefinition::Definition(api_def) => {
                Some(api_def)
            }
            apidefinition::create_api_definition_request::ApiDefinition::Openapi(_) => None,
        }
    }
}

impl HasProtoApiDefinition for apidefinition::update_api_definition_request::ApiDefinition {
    fn proto_api_definition_kind(&self) -> ProtoApiDefinitionKind {
        match self {
            apidefinition::update_api_definition_request::ApiDefinition::Definition(_) => {
                ProtoApiDefinitionKind::Golem
            }
            apidefinition::update_api_definition_request::ApiDefinition::Openapi(_) => {
                ProtoApiDefinitionKind::OpenAPI
            }
        }
    }

    fn api_definition(&self) -> Option<&apidefinition::ApiDefinition> {
        match self {
            apidefinition::update_api_definition_request::ApiDefinition::Definition(api_def) => {
                Some(api_def)
            }
            apidefinition::update_api_definition_request::ApiDefinition::Openapi(_) => None,
        }
    }
}

pub fn proto_api_definition_kind_string<T: HasProtoApiDefinition>(
    opt_t: &Option<T>,
) -> Option<String> {
    opt_t.as_ref().map(|t| match t.proto_api_definition_kind() {
        ProtoApiDefinitionKind::Golem => "golem".to_owned(),
        ProtoApiDefinitionKind::OpenAPI => "openapi".to_owned(),
    })
}

pub fn proto_api_definition_id_string<T: HasProtoApiDefinition>(
    api_definition: &Option<T>,
) -> Option<String> {
    api_definition
        .as_ref()
        .and_then(|d| d.api_definition())
        .and_then(|d| d.id.clone())
        .map(|id| id.value)
}

pub fn proto_api_definition_version_string<T: HasProtoApiDefinition>(
    api_definition: &Option<T>,
) -> Option<String> {
    api_definition
        .as_ref()
        .and_then(|d| d.api_definition())
        .map(|d| d.version.clone())
}

pub fn proto_api_definition_draft_string<T: HasProtoApiDefinition>(
    api_definition: &Option<T>,
) -> Option<String> {
    api_definition
        .as_ref()
        .and_then(|d| d.api_definition())
        .map(|d| d.draft.to_string())
}
