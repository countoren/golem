use super::*;
use crate::config::{CompileWorkerConfig, ComponentServiceConfig, UploadWorkerConfig};
use crate::model::*;
use async_trait::async_trait;
use golem_common::model::ComponentId;
use golem_worker_executor_base::services::compiled_component::CompiledComponentService;
use std::sync::Arc;
use tokio::sync::mpsc;
use wasmtime::Engine;

#[async_trait]
pub trait CompilationService {
    async fn enqueue_compilation(
        &self,
        component_id: ComponentId,
        component_version: u64,
    ) -> Result<(), CompilationError>;
}

#[derive(Clone)]
pub struct ComponentCompilationServiceImpl {
    queue: mpsc::Sender<CompilationRequest>,
}

impl ComponentCompilationServiceImpl {
    pub fn new(
        upload_worker: UploadWorkerConfig,
        compile_worker: CompileWorkerConfig,
        component_service: ComponentServiceConfig,

        engine: Engine,

        compiled_component_service: Arc<dyn CompiledComponentService + Send + Sync>,
    ) -> Self {
        let (compile_tx, compile_rx) = mpsc::channel(100);
        let (upload_tx, upload_rx) = mpsc::channel(100);

        CompileWorker::start(
            component_service.uri(),
            component_service.access_token,
            compile_worker,
            engine.clone(),
            compiled_component_service.clone(),
            upload_tx,
            compile_rx,
        );

        UploadWorker::start(upload_worker, compiled_component_service.clone(), upload_rx);

        Self { queue: compile_tx }
    }
}

#[async_trait]
impl CompilationService for ComponentCompilationServiceImpl {
    async fn enqueue_compilation(
        &self,
        component_id: ComponentId,
        component_version: u64,
    ) -> Result<(), CompilationError> {
        tracing::info!(
            "Enqueueing compilation for component {}@{}",
            component_id,
            component_version
        );
        let request = CompilationRequest {
            component: ComponentWithVersion {
                id: component_id,
                version: component_version,
            },
        };
        self.queue.send(request).await?;
        crate::metrics::increment_queue_length();
        Ok(())
    }
}