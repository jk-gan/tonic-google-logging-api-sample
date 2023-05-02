use google::logging::v2::{
    logging_service_v2_client::LoggingServiceV2Client, TailLogEntriesRequest,
};
use tokio_stream::StreamExt;
use tonic::{codegen::InterceptedService, metadata::MetadataValue, transport::Channel, Request};

#[rustfmt::skip]
#[path = "gcloud/api"]
pub mod google {
    #[path = ""]
    pub mod logging {
        #[path = "google.logging.r#type.rs"]
        pub mod r#type;
        #[path = "google.logging.v2.rs"]
        pub mod v2;
    }
    #[path = "google.api.rs"]
    pub mod api;
    #[path = "google.rpc.rs"]
    pub mod rpc;
}

// I'm using these scopes:
// - "https://www.googleapis.com/auth/cloud-platform"
// - "https://www.googleapis.com/auth/cloud-platform.read-only"
// - "https://www.googleapis.com/auth/logging.admin"
// - "https://www.googleapis.com/auth/logging.read"
// - "https://www.googleapis.com/auth/logging.write"
const ACCESS_TOKEN: &str = "";
const RESOURCE_NAME: &str = "projects/xxx";

#[tokio::main]
async fn main() {
    let bearer_token = format!("Bearer {}", ACCESS_TOKEN);
    let header_value: MetadataValue<_> = bearer_token.parse().unwrap();

    let channel = Channel::from_static("https://logging.googleapis.com")
        .connect()
        .await
        .unwrap();

    let mut service: LoggingServiceV2Client<InterceptedService<Channel, _>> =
        LoggingServiceV2Client::with_interceptor(channel, move |mut req: Request<()>| {
            let metadata_map = req.metadata_mut();
            metadata_map.insert("authorization", header_value.clone());

            Ok(req)
        });

    let stream = tokio_stream::iter([1]).map(|_| TailLogEntriesRequest {
        resource_names: vec![RESOURCE_NAME.to_string()],
        ..Default::default()
    });

    let mut response = service.tail_log_entries(stream).await.unwrap().into_inner();

    loop {
        let received = response.message().await.unwrap();
        println!("\treceived message: `{:?}`", received);
    }
}
