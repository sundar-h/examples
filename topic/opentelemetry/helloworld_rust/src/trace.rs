pub mod helloworld {
    tonic::include_proto!("helloworld");
}

use std::{convert::Infallible, error::Error};

use crate::trace::helloworld::greeter_server::Greeter;
use http::{Request, Response};
use hyper::{client::connect::dns::Name, Body};
use opentelemetry::sdk::{
    propagation::TraceContextPropagator,
    trace::{self, IdGenerator, Sampler},
    Resource,
};
use opentelemetry::{global, propagation::Extractor, KeyValue};
use tonic::codegen::BoxFuture;
use tonic::{body::BoxBody, transport::NamedService};
use tower_service::Service;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use tracing_subscriber::prelude::*;

struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);

impl<'a> Extractor for MetadataMap<'a> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}

pub fn with_grpc_span(metadata: &tonic::metadata::MetadataMap) {
    println!("grpc: {:?}", metadata);
    let parent_cx = global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(metadata)));
    println!("{:#?}", parent_cx);
    tracing::Span::current().set_parent(parent_cx);
}

pub fn with_http_span(headers: &http::HeaderMap) {
    let ref metadata = tonic::metadata::MetadataMap::from_headers(headers.clone());
    println!("http: {:?}", metadata);

    let parent_cx = global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(metadata)));
    println!("{:#?}", parent_cx);
    // tracing::Span::current().set_parent(parent_cx);
    let span = tracing::Span::current();
    span.set_parent(parent_cx);
}

pub fn tracing_init() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_agent_endpoint("127.0.0.1:6831")
        .with_service_name("rust-greeter")
        .with_max_packet_size(9_216)
        .with_auto_split_batch(true)
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(IdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![
                    KeyValue::new("key", "value"),
                    KeyValue::new("process_key", "process_value"),
                ])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Could not create async Tracer");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("TRACE"))
        // .with(EnvFilter::from_default_env())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()
        .unwrap();
}
