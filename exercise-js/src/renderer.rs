use crate::{
    css,
    dom::{Node, NodeType},
    javascript::{renderapi::RendererAPI, JavaScriptRuntime},
    layout::to_layout_box,
    render::{to_element_container, ElementContainer},
    style::to_styled_node,
};
use cursive::{
    direction::Direction,
    event::{AnyCb, Event, EventResult},
    view::{Selector, View, ViewNotFound},
    Rect, Vec2,
};

use cursive::CbSink;
use std::{cell::RefCell, rc::Rc};

pub struct Renderer {
    view: ElementContainer, // 現在画面に描画している実際の Cursive ビュー
    document_element: Rc<RefCell<Box<Node>>>, // view の元となっている DOM ツリー
    js_runtime_instance: JavaScriptRuntime, // Cursive ビューに関連した JavaScript 処理系のインスタンス
}

// Renderer を Cursive のビューとして利用するための実装
impl View for Renderer {
    fn draw(&self, printer: &cursive::Printer) {
        self.view.draw(printer)
    }

    fn layout(&mut self, v: Vec2) {
        self.view.layout(v)
    }

    fn needs_relayout(&self) -> bool {
        self.view.needs_relayout()
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.view.required_size(constraint)
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        self.view.on_event(e)
    }

    fn call_on_any<'a>(&mut self, s: &Selector<'_>, cb: AnyCb<'a>) {
        self.view.call_on_any(s, cb)
    }

    fn focus_view(&mut self, s: &Selector<'_>) -> Result<(), ViewNotFound> {
        self.view.focus_view(s)
    }

    fn take_focus(&mut self, source: Direction) -> bool {
        self.view.take_focus(source)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        self.view.important_area(view_size)
    }

    fn type_name(&self) -> &'static str {
        self.view.type_name()
    }
}