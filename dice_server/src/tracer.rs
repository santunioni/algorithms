use opentelemetry::global;
use opentelemetry::global::{BoxedTracer, tracer};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_stdout::{LogExporter, MetricExporter, SpanExporter};
use std::sync::OnceLock;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

pub fn global_tracer() -> &'static BoxedTracer {
    static TRACER: OnceLock<BoxedTracer> = OnceLock::new();
    TRACER.get_or_init(|| tracer("dice_server"))
}

pub fn init_otel() -> OtelProviders {
    let logger_provider = logger_provider();
    let otel_layer =
        OpenTelemetryTracingBridge::new(&logger_provider).with_filter(filter_for_otel_layer());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .pretty() // We must use json() in production
        .with_filter(EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(fmt_layer)
        .init();

    let tracer_provider = tracer_provider();
    global::set_tracer_provider(tracer_provider.clone());

    let meter_provider = meter_provider();
    global::set_meter_provider(meter_provider.clone());

    OtelProviders {
        tracer_provider,
        meter_provider,
        logger_provider,
    }
}

/// To prevent a telemetry-induced-telemetry loop, OpenTelemetry's own internal
/// logging is properly suppressed. However, logs emitted by external components
/// (such as reqwest, tonic, etc.) are not suppressed as they do not propagate
/// OpenTelemetry context. Until this issue is addressed
/// (https://github.com/open-telemetry/opentelemetry-rust/issues/2877),
/// filtering like this is the best way to suppress such logs.
///
/// The filter levels are set as follows:
/// - Allow `info` level and above by default.
/// - Completely restrict logs from `hyper`, `tonic`, `h2`, and `reqwest`.
///
/// Note: This filtering will also drop logs from these components even when
/// they are used outside of the OTLP Exporter.
fn filter_for_otel_layer() -> EnvFilter {
    EnvFilter::from_default_env()
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap())
}

pub struct OtelProviders {
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
    logger_provider: SdkLoggerProvider,
}

impl Drop for OtelProviders {
    fn drop(&mut self) {
        let mut shutdown_errors = Vec::new();
        if let Err(e) = self.tracer_provider.shutdown() {
            shutdown_errors.push(format!("tracer provider: {e}"));
        }

        if let Err(e) = self.meter_provider.shutdown() {
            shutdown_errors.push(format!("meter provider: {e}"));
        }

        if let Err(e) = self.logger_provider.shutdown() {
            shutdown_errors.push(format!("logger provider: {e}"));
        }

        if !shutdown_errors.is_empty() {
            eprintln!(
                "Failed to shutdown providers:{}",
                shutdown_errors.join("\n")
            );
        }
    }
}

fn logger_provider() -> SdkLoggerProvider {
    let exporter = LogExporter::default();

    SdkLoggerProvider::builder()
        .with_resource(get_resource())
        .with_batch_exporter(exporter)
        .build()
}

fn tracer_provider() -> SdkTracerProvider {
    // Conditionally use datadog here
    // Remember to set_text_map_propagator(DatadogPropagator::default()) to push baggage forward
    // To read baggage, use for example (http):
    // let parent_ctx =  get_text_map_propagator(|propagator| propagator.extract(&HeaderExtractor(request.headers())));
    // sqs should have something similar

    // The SpanExporter::default() will echo spans to stdout, useful for debugging
    SdkTracerProvider::builder()
        .with_resource(get_resource())
        .with_batch_exporter(SpanExporter::default())
        .build()

    // Uncomment the following lines to enable Datadog tracing
    // let mut tracer_builder = opentelemetry_datadog::new_pipeline()
    //     .with_api_version(opentelemetry_datadog::ApiVersion::Version05)
    //     .with_service_name(DD_SERVICE.to_owned())
    //     .with_trace_config(Config::default());
    //
    // if let Some(agent) = DD_AGENT_ENDPOINT.as_ref() {
    //     tracer_builder = tracer_builder.with_agent_endpoint(agent);
    // }
    //
    // if let Some(version) = DD_VERSION.as_ref() {
    //     tracer_builder = tracer_builder.with_version(version);
    // }
    //
    // let client = reqwest::ClientBuilder::new()
    //     .pool_max_idle_per_host(0)
    //     .build()
    //     .unwrap();
    //
    // global::set_tracer_provider(
    //     tracer_builder
    //         .with_http_client::<reqwest::Client>(client)
    //         .install_batch()
    //         .expect("Could not conclude Datadog batch tracer pipeline"),
    // );
}

fn meter_provider() -> SdkMeterProvider {
    // How can we use Prometheus here?
    // Or should we export to datadog agent directly? (prometheus would export to datadog anyway)
    let exporter = MetricExporter::builder().build();

    SdkMeterProvider::builder()
        .with_periodic_exporter(exporter)
        .with_resource(get_resource())
        .build()
}

fn get_resource() -> Resource {
    static RESOURCE: OnceLock<Resource> = OnceLock::new();
    RESOURCE
        .get_or_init(|| Resource::builder().with_service_name("dice_server").build())
        .clone()
}
