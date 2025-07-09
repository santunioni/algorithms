use crate::tracer::global_tracer;
use opentelemetry::Context;
use opentelemetry::context::FutureExt;
use opentelemetry::trace::{Span, TraceContextExt, Tracer};
use std::borrow::Cow;

pub trait ExtendedFuture: Future + Sized {
    #[inline]
    fn in_child_span<T>(self, name: T) -> impl Future<Output = Self::Output>
    where
        T: Into<Cow<'static, str>>,
    {
        self.in_span(global_tracer().start(name))
    }

    #[inline]
    fn in_current_span(self) -> impl Future<Output = Self::Output> {
        self.with_context(Context::current())
    }

    #[inline]
    fn in_span<T>(self, span: T) -> impl Future<Output = Self::Output>
    where
        T: Span + Send + Sync + 'static,
    {
        self.with_context(Context::current_with_span(span))
    }
}

impl<T: Future + Sized> ExtendedFuture for T {}
