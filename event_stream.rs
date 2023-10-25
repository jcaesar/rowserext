use futures::channel::mpsc;
use futures::stream::{FusedStream, Stream};
use gloo::events::EventListener;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasm_bindgen::UnwrapThrowExt;
use web_sys::EventTarget;

pub struct EventStream {
    receiver: mpsc::UnboundedReceiver<()>,
    #[allow(unused)]
    listener: EventListener,
}

impl EventStream {
    pub fn new(target: &EventTarget, event_type: &'static str) -> Self {
        let (sender, receiver) = mpsc::unbounded();
        let listener = EventListener::new(&target, event_type, move |_event| {
            sender.unbounded_send(()).unwrap_throw();
        });
        Self { receiver, listener }
    }
}

impl Stream for EventStream {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}

impl FusedStream for EventStream {
    fn is_terminated(&self) -> bool {
        false
    }
}
