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

pub mod context;
pub mod services;

use std::sync::Arc;

use async_trait::async_trait;
use golem_worker_executor_base::durable_host::DurableWorkerCtx;
use golem_worker_executor_base::preview2::golem;
use golem_worker_executor_base::services::active_workers::ActiveWorkers;
use golem_worker_executor_base::services::blob_store::BlobStoreService;
use golem_worker_executor_base::services::component::ComponentService;
use golem_worker_executor_base::services::events::Events;
use golem_worker_executor_base::services::golem_config::GolemConfig;
use golem_worker_executor_base::services::key_value::KeyValueService;
use golem_worker_executor_base::services::oplog::OplogService;
use golem_worker_executor_base::services::promise::PromiseService;
use golem_worker_executor_base::services::recovery::RecoveryManagementDefault;
use golem_worker_executor_base::services::rpc::{DirectWorkerInvocationRpc, RemoteInvocationRpc};
use golem_worker_executor_base::services::scheduler::SchedulerService;
use golem_worker_executor_base::services::shard::ShardService;
use golem_worker_executor_base::services::shard_manager::ShardManagerService;
use golem_worker_executor_base::services::worker::WorkerService;
use golem_worker_executor_base::services::worker_activator::WorkerActivator;
use golem_worker_executor_base::services::worker_enumeration::{
    RunningWorkerEnumerationService, WorkerEnumerationService,
};
use golem_worker_executor_base::services::worker_proxy::WorkerProxy;
use golem_worker_executor_base::services::All;
use golem_worker_executor_base::wasi_host::create_linker;
use golem_worker_executor_base::Bootstrap;
use prometheus::Registry;
use tokio::runtime::Handle;
use tracing::info;
use wasmtime::component::Linker;
use wasmtime::Engine;

use crate::context::Context;
use crate::services::AdditionalDeps;

struct ServerBootstrap {}

#[async_trait]
impl Bootstrap<Context> for ServerBootstrap {
    fn create_active_workers(&self, _golem_config: &GolemConfig) -> Arc<ActiveWorkers<Context>> {
        Arc::new(ActiveWorkers::<Context>::unbounded())
    }

    async fn create_services(
        &self,
        active_workers: Arc<ActiveWorkers<Context>>,
        engine: Arc<Engine>,
        linker: Arc<Linker<Context>>,
        runtime: Handle,
        component_service: Arc<dyn ComponentService + Send + Sync>,
        shard_manager_service: Arc<dyn ShardManagerService + Send + Sync>,
        worker_service: Arc<dyn WorkerService + Send + Sync>,
        worker_enumeration_service: Arc<dyn WorkerEnumerationService + Send + Sync>,
        running_worker_enumeration_service: Arc<dyn RunningWorkerEnumerationService + Send + Sync>,
        promise_service: Arc<dyn PromiseService + Send + Sync>,
        golem_config: Arc<GolemConfig>,
        shard_service: Arc<dyn ShardService + Send + Sync>,
        key_value_service: Arc<dyn KeyValueService + Send + Sync>,
        blob_store_service: Arc<dyn BlobStoreService + Send + Sync>,
        worker_activator: Arc<dyn WorkerActivator + Send + Sync>,
        oplog_service: Arc<dyn OplogService + Send + Sync>,
        scheduler_service: Arc<dyn SchedulerService + Send + Sync>,
        worker_proxy: Arc<dyn WorkerProxy + Send + Sync>,
        events: Arc<Events>,
    ) -> anyhow::Result<All<Context>> {
        let additional_deps = AdditionalDeps {};

        let rpc = Arc::new(DirectWorkerInvocationRpc::new(
            Arc::new(RemoteInvocationRpc::new(worker_proxy.clone())),
            active_workers.clone(),
            engine.clone(),
            linker.clone(),
            runtime.clone(),
            component_service.clone(),
            worker_service.clone(),
            worker_enumeration_service.clone(),
            running_worker_enumeration_service.clone(),
            promise_service.clone(),
            golem_config.clone(),
            shard_service.clone(),
            shard_manager_service.clone(),
            key_value_service.clone(),
            blob_store_service.clone(),
            oplog_service.clone(),
            scheduler_service.clone(),
            worker_activator.clone(),
            events.clone(),
            additional_deps.clone(),
        ));
        let recovery_management = Arc::new(RecoveryManagementDefault::new(
            active_workers.clone(),
            engine.clone(),
            linker.clone(),
            runtime.clone(),
            component_service.clone(),
            worker_service.clone(),
            worker_enumeration_service.clone(),
            running_worker_enumeration_service.clone(),
            oplog_service.clone(),
            promise_service.clone(),
            scheduler_service.clone(),
            key_value_service.clone(),
            blob_store_service.clone(),
            rpc.clone(),
            worker_activator.clone(),
            worker_proxy.clone(),
            events.clone(),
            golem_config.clone(),
            additional_deps.clone(),
        ));
        rpc.set_recovery_management(recovery_management.clone());

        Ok(All::new(
            active_workers,
            engine,
            linker,
            runtime.clone(),
            component_service,
            shard_manager_service,
            worker_service,
            worker_enumeration_service,
            running_worker_enumeration_service,
            promise_service,
            golem_config.clone(),
            shard_service,
            key_value_service,
            blob_store_service,
            oplog_service,
            recovery_management,
            rpc,
            scheduler_service,
            worker_activator.clone(),
            worker_proxy.clone(),
            events.clone(),
            additional_deps,
        ))
    }

    fn create_wasmtime_linker(&self, engine: &Engine) -> anyhow::Result<Linker<Context>> {
        let mut linker =
            create_linker::<Context, DurableWorkerCtx<Context>>(engine, |x| &mut x.durable_ctx)?;
        golem::api::host::add_to_linker::<Context, DurableWorkerCtx<Context>>(&mut linker, |x| {
            &mut x.durable_ctx
        })?;
        golem_wasm_rpc::golem::rpc::types::add_to_linker::<Context, DurableWorkerCtx<Context>>(
            &mut linker,
            |x| &mut x.durable_ctx,
        )?;
        Ok(linker)
    }
}

pub async fn run(
    golem_config: GolemConfig,
    prometheus_registry: Registry,
    runtime: Handle,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Golem Worker Executor starting up...");
    Ok(ServerBootstrap {}
        .run(golem_config, prometheus_registry, runtime)
        .await?)
}
