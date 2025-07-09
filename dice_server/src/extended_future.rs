use opentelemetry::Context;
use opentelemetry::trace::{Span, TraceContextExt};

pub trait TracedFuture: Future + Sized {
    /// Wraps a future with a profiler that tracks the future's execution.
    #[inline]
    fn run_in_span<S: Span + Send + Sync + 'static>(
        self,
        span: S,
    ) -> impl Future<Output = Self::Output> {
        async move {
            let cx = Context::current_with_span(span);
            let _guard = cx.clone().attach();
            self.await
        }
    }
}

impl<T: Future + Sized> TracedFuture for T {}
