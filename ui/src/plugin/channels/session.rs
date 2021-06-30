use std::rc::Rc;

use nativeshell::codec::{MethodCall, MethodCallReply, Value};
use nativeshell::shell::{Context, EngineHandle, MethodCallHandler, MethodChannel};
use protobuf::ProtobufEnum;

use mizer_api::handlers::SessionHandler;
use mizer_api::models::TransportState;
use mizer_api::RuntimeApi;

use crate::plugin::channels::MethodReplyExt;

#[derive(Clone)]
pub struct SessionChannel<R: RuntimeApi> {
    handler: SessionHandler<R>,
}

impl<R: RuntimeApi + 'static> MethodCallHandler for SessionChannel<R> {
    fn on_method_call(
        &mut self,
        call: MethodCall<Value>,
        resp: MethodCallReply<Value>,
        _: EngineHandle,
    ) {
        match call.method.as_str() {
            "saveProject" => {
                match self.save_project() {
                    Ok(()) => resp.send_ok(Value::Null),
                    Err(e) => resp.respond_error(e),
                }
            }
            _ => resp.not_implemented(),
        }
    }
}

impl<R: RuntimeApi + 'static> SessionChannel<R> {
    pub fn new(handler: SessionHandler<R>) -> Self {
        Self { handler }
    }

    pub fn channel(self, context: Rc<Context>) -> MethodChannel {
        MethodChannel::new(context, "mizer.live/session", self)
    }

    fn save_project(&self) -> anyhow::Result<()> {
        self.handler.save_project()
    }
}