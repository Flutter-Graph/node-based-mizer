use nativeshell::codec::{MethodCall, MethodCallReply, Value};
use nativeshell::shell::{Context, EngineHandle, MethodCallHandler, MethodChannel};

use mizer_api::handlers::EffectsHandler;
use mizer_api::models::Effects;
use mizer_api::RuntimeApi;

use crate::plugin::channels::MethodReplyExt;

#[derive(Clone)]
pub struct EffectsChannel<R> {
    handler: EffectsHandler<R>,
}

impl<R: RuntimeApi + 'static> MethodCallHandler for EffectsChannel<R> {
    fn on_method_call(
        &mut self,
        call: MethodCall<Value>,
        resp: MethodCallReply<Value>,
        _: EngineHandle,
    ) {
        log::debug!("mizer.live/layouts -> {}", call.method);
        match call.method.as_str() {
            "getEffects" => {
                let effects = self.get_effects();

                resp.respond_msg(effects);
            }
            "deleteEffect" => {
                if let Value::I64(id) = call.args {
                    self.handler.delete_effect(id as u32);

                    resp.send_ok(Value::Null)
                }
            }
            _ => resp.not_implemented(),
        }
    }
}

impl<R: RuntimeApi + 'static> EffectsChannel<R> {
    pub fn new(handler: EffectsHandler<R>) -> Self {
        Self { handler }
    }

    pub fn channel(self, context: Context) -> MethodChannel {
        MethodChannel::new(context, "mizer.live/effects", self)
    }

    pub fn get_effects(&self) -> Effects {
        self.handler.get_effects()
    }
}
