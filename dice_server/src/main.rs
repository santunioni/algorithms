mod extended_future;
mod tracer;

use std::convert::Infallible;
use std::net::SocketAddr;

use crate::tracer::{get_tracer, init_tracer_provider, install_tracing_library};
use http_body_util::Full;
use hyper::Method;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use opentelemetry::trace::{Span, SpanBuilder, SpanKind, Status, TraceContextExt, Tracer};
use opentelemetry::{InstrumentationScope, KeyValue, global};
use rand::Rng;
use rayon::ThreadPoolBuilder;
use tokio::net::TcpListener;
use tokio::runtime::Builder;

async fn roll_dice(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let common_scope_attributes = vec![KeyValue::new("scope-key", "scope-value")];
    let scope = InstrumentationScope::builder("basic")
        .with_version("1.0")
        .with_attributes(common_scope_attributes)
        .build();

    let tracer = global::tracer_with_scope(scope.clone());
    let meter = global::meter_with_scope(scope);

    tracer.in_span("Main operation", |cx| {
        let span = cx.span();
        span.add_event(
            "Nice operation!".to_string(),
            vec![KeyValue::new("bogons", 100)],
        );
        span.set_attribute(KeyValue::new("another.key", "yes"));

        tracing::error!(name: "my-event-inside-span", target: "my-target", "hello from {}. My price is {}. I am also inside a Span!", "banana", 2.99);

        tracer.in_span("Sub operation...", |cx| {
            let span = cx.span();
            span.set_attribute(KeyValue::new("another.key", "yes"));
            span.add_event("Sub span event", vec![]);
        });
    });

    tracing::error!(name: "my-event", target: "my-target", "hello from {}. My price is {}", "apple", 1.99);

    let random_number = rand::rng().random_range(1..=6);
    tracing::error!(random_number = random_number, "Found number");
    Ok(Response::new(Full::new(Bytes::from(
        random_number.to_string(),
    ))))
}

async fn handle(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let tracer = get_tracer();

    let mut span = SpanBuilder::from_name(format!("{} {}", req.method(), req.uri().path()))
        .with_kind(SpanKind::Server)
        .start(tracer);

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/rolldice") => roll_dice(req).await,
        _ => {
            span.set_status(Status::Ok);
            Ok(Response::builder()
                .status(404)
                .body(Full::new(Bytes::from("Not Found")))
                .unwrap())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    ThreadPoolBuilder::new().num_threads(10).build_global()?;

    let tokio_runtime = Builder::new_multi_thread()
        .worker_threads(10)
        .enable_all()
        .build()?;

    tokio_runtime.block_on(async {
        init_tracer_provider();
        install_tracing_library();
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(handle))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    })
}
