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
    let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    // To prevent a telemetry-induced-telemetry loop, OpenTelemetry's own internal
    // logging is properly suppressed. However, logs emitted by external components
    // (such as reqwest, tonic, etc.) are not suppressed as they do not propagate
    // OpenTelemetry context. Until this issue is addressed
    // (https://github.com/open-telemetry/opentelemetry-rust/issues/2877),
    // filtering like this is the best way to suppress such logs.
    //
    // The filter levels are set as follows:
    // - Allow `info` level and above by default.
    // - Completely restrict logs from `hyper`, `tonic`, `h2`, and `reqwest`.
    //
    // Note: This filtering will also drop logs from these components even when
    // they are used outside of the OTLP Exporter.
    let filter_otel = EnvFilter::from_default_env()
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    let otel_layer = otel_layer.with_filter(filter_otel);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .pretty()
        .with_filter(EnvFilter::from_default_env());

    // Initialize the tracing subscriber with the OpenTelemetry layer and the
    // Fmt layer.
    tracing_subscriber::registry()
        .with(otel_layer)
        .with(fmt_layer)
        .init();

    // At this point Logs (OTel Logs and Fmt Logs) are initialized, which will
    // allow internal-logs from Tracing/Metrics initializer to be captured.

    let tracer_provider = tracer_provider();
    // Set the global tracer provider using a clone of the tracer_provider.
    // Setting global tracer provider is required if other parts of the application
    // uses global::tracer() or global::tracer_with_version() to get a tracer.
    // Cloning simply creates a new reference to the same tracer provider. It is
    // important to hold on to the tracer_provider here, so as to invoke
    // shutdown on it when application ends.
    global::set_tracer_provider(tracer_provider.clone());

    let meter_provider = meter_provider();
    // Set the global meter provider using a clone of the meter_provider.
    // Setting global meter provider is required if other parts of the application
    // uses global::meter() or global::meter_with_version() to get a meter.
    // Cloning simply creates a new reference to the same meter provider. It is
    // important to hold on to the meter_provider here, to invoke
    // shutdown on it when application ends.
    global::set_meter_provider(meter_provider.clone());

    OtelProviders {
        tracer_provider,
        meter_provider,
        logger_provider,
    }
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
    let exporter = SpanExporter::default();
    SdkTracerProvider::builder()
        .with_resource(get_resource())
        .with_batch_exporter(exporter)
        .build()
}

fn meter_provider() -> SdkMeterProvider {
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
