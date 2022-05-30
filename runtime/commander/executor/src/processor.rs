use crate::{CommandExecutor, InMainLoopExecutionWorker};
use mizer_processing::*;

pub struct CommandProcessor {
    executor: CommandExecutor,
    worker: InMainLoopExecutionWorker,
}

impl std::fmt::Debug for CommandProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandProcessor").finish()
    }
}

impl CommandProcessor {
    pub(crate) fn new(executor: CommandExecutor, worker: InMainLoopExecutionWorker) -> Self {
        Self { executor, worker }
    }
}

impl Processor for CommandProcessor {
    #[tracing::instrument]
    fn pre_process(&mut self, injector: &mut Injector, _: ClockFrame) {
        if let Err(err) = self.worker.process_callbacks(&mut self.executor, injector) {
            log::error!("Error processing commands {:?}", err);
        }
    }
}