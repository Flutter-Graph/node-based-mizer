use std::hash::Hash;

use crate::*;

pub struct NoopDebugUi;

impl DebugUi for NoopDebugUi {
    type RenderHandle<'a, 'b> = ();
    type DrawHandle<'a> = ();
    type TextureMap = ();

    fn pre_render(&mut self) -> Self::RenderHandle<'_, '_> {
        ()
    }

    fn render(&mut self) {}
}

impl DebugUiRenderHandle<'_> for () {
    type DrawHandle<'b> = ();
    type TextureMap = ();

    fn draw(&mut self, _call: impl FnOnce(&mut Self::DrawHandle<'_>, &mut Self::TextureMap)) {}
}

impl DebugUiDrawHandle<'_> for () {
    type Response = ();
    type DrawHandle<'b> = ();
    type TextureMap = ();

    fn horizontal(&mut self, _cb: impl FnOnce(&mut Self::DrawHandle<'_>)) {}

    fn button(&mut self, _text: impl Into<String>) -> Self::Response {
        ()
    }

    fn heading(&mut self, _text: impl Into<String>) {}

    fn label(&mut self, _text: impl Into<String>) {}

    fn collapsing_header(
        &mut self,
        _title: impl Into<String>,
        _add_content: impl FnOnce(&mut Self::DrawHandle<'_>),
    ) {
    }

    fn columns(&mut self, _count: usize, _add_contents: impl FnOnce(&mut [Self::DrawHandle<'_>])) {}

    fn image<I: Hash>(&mut self, _image_id: I, _data: &[u8], _textures: &mut Self::TextureMap) {}
}

impl DebugUiResponse for () {
    fn clicked(&self) -> bool {
        false
    }
}