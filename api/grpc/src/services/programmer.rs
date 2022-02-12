use futures::StreamExt;
use grpc::{Metadata, ServerRequestSingle, ServerResponseSink, ServerResponseUnarySink};

use mizer_api::handlers::ProgrammerHandler;
use mizer_api::RuntimeApi;

use crate::protos::*;

impl<R: RuntimeApi + 'static> ProgrammerApi for ProgrammerHandler<R> {
    fn subscribe_to_programmer(
        &self,
        req: ServerRequestSingle<SubscribeProgrammerRequest>,
        mut resp: ServerResponseSink<ProgrammerState>,
    ) -> grpc::Result<()> {
        let mut stream = self.state_stream();
        req.loop_handle().spawn(async move {
            while let Some(m) = stream.next().await {
                resp.send_data(m)?;
            }
            resp.send_trailers(Metadata::new())
        });

        Ok(())
    }

    fn write_control(
        &self,
        req: ServerRequestSingle<WriteControlRequest>,
        resp: ServerResponseUnarySink<WriteControlResponse>,
    ) -> grpc::Result<()> {
        self.write_control(req.message);

        resp.finish(Default::default())
    }

    fn select_fixtures(
        &self,
        req: ServerRequestSingle<SelectFixturesRequest>,
        resp: ServerResponseUnarySink<SelectFixturesResponse>,
    ) -> grpc::Result<()> {
        self.select_fixtures(req.message.fixtures.into_vec());

        resp.finish(Default::default())
    }

    fn clear(
        &self,
        _: ServerRequestSingle<ClearRequest>,
        resp: ServerResponseUnarySink<ClearResponse>,
    ) -> grpc::Result<()> {
        self.clear();

        resp.finish(Default::default())
    }

    fn highlight(
        &self,
        req: ServerRequestSingle<HighlightRequest>,
        resp: ServerResponseUnarySink<HighlightResponse>,
    ) -> grpc::Result<()> {
        self.highlight(req.message.highlight);

        resp.finish(Default::default())
    }

    fn store(
        &self,
        req: ServerRequestSingle<StoreRequest>,
        resp: ServerResponseUnarySink<super::programmer::StoreResponse>,
    ) -> grpc::Result<()> {
        self.store(req.message.sequence_id, req.message.store_mode);

        resp.finish(Default::default())
    }

    fn get_presets(
        &self,
        req: ServerRequestSingle<PresetsRequest>,
        resp: ServerResponseUnarySink<Presets>,
    ) -> grpc::Result<()> {
        let presets = self.get_presets();

        resp.finish(presets)
    }

    fn call_preset(
        &self,
        req: ServerRequestSingle<PresetId>,
        resp: ServerResponseUnarySink<CallPresetResponse>,
    ) -> grpc::Result<()> {
        self.call_preset(req.message);

        resp.finish(Default::default())
    }

    fn get_groups(
        &self,
        req: ServerRequestSingle<GroupsRequest>,
        resp: ServerResponseUnarySink<Groups>,
    ) -> grpc::Result<()> {
        let groups = self.get_groups();

        resp.finish(groups)
    }

    fn select_group(
        &self,
        req: ServerRequestSingle<super::programmer::SelectGroupRequest>,
        resp: ServerResponseUnarySink<super::programmer::SelectGroupResponse>,
    ) -> grpc::Result<()> {
        self.select_group(req.message.id);

        resp.finish(Default::default())
    }

    fn add_group(
        &self,
        req: ServerRequestSingle<AddGroupRequest>,
        resp: ServerResponseUnarySink<Group>,
    ) -> grpc::Result<()> {
        let group = self.add_group(req.message.name);

        resp.finish(group)
    }

    fn assign_fixtures_to_group(
        &self,
        req: ServerRequestSingle<AssignFixturesToGroupRequest>,
        resp: ServerResponseUnarySink<AssignFixturesToGroupResponse>,
    ) -> grpc::Result<()> {
        self.assign_fixtures_to_group(req.message.id, req.message.fixtures.into_vec());

        resp.finish(Default::default())
    }
}
