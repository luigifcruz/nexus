use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::trace;

use nexus_common::proto::generic::Ack;
use nexus_common::proto::nexus::nexus_server::Nexus;
use nexus_common::proto::nexus::{
    CreateImageRequest, CreateImageResponse, CreateObservationRequest, CreateObservationResponse,
    DestroyImageRequest, DestroyImageResponse, DestroyObservationRequest,
    DestroyObservationResponse, ListImagesRequest, ListImagesResponse, ListObservationsRequest,
    ListObservationsResponse, ListReplicantsRequest, ListReplicantsResponse, NexusSignallerRequest,
    NexusSignallerResponse,
};

use crate::domain::services::{
    ImageService, InstanceService, ObservationService, ReplicantService,
};
use crate::transport::connections::SubscriptionManager;

pub use nexus_common::proto::nexus::nexus_server::NexusServer;
pub use nexus_common::proto::nexus::FILE_DESCRIPTOR_SET as NexusFileDescriptorSet;

pub struct NexusHandler {
    observation_service: Arc<ObservationService>,
    replicant_service: Arc<ReplicantService>,
    instance_service: Arc<InstanceService>,
    image_service: Arc<ImageService>,
    subscription_manager: Arc<SubscriptionManager>,
}

impl NexusHandler {
    pub fn new(
        observation_service: Arc<ObservationService>,
        replicant_service: Arc<ReplicantService>,
        instance_service: Arc<InstanceService>,
        image_service: Arc<ImageService>,
        subscription_manager: Arc<SubscriptionManager>,
    ) -> Self {
        Self {
            observation_service,
            replicant_service,
            instance_service,
            image_service,
            subscription_manager,
        }
    }
}

#[tonic::async_trait]
impl Nexus for NexusHandler {
    async fn create_image(
        &self,
        request: Request<CreateImageRequest>,
    ) -> Result<Response<CreateImageResponse>, Status> {
        let inner = request.into_inner();

        if inner.validate_only {
            let image = crate::domain::entities::Image::new(
                inner.image_id,
                inner.docker_image,
                inner.docker_entrypoint,
                inner.docker_args,
                inner.docker_env,
            );

            return match image.validate() {
                Ok(_) => Ok(Response::new(CreateImageResponse {
                    ack: Some(Ack {
                        success: true,
                        message: "Validation successful".to_string(),
                    }),
                })),
                Err(e) => Ok(Response::new(CreateImageResponse {
                    ack: Some(Ack {
                        success: false,
                        message: e,
                    }),
                })),
            };
        }

        // Create the image
        match self
            .image_service
            .create_image(
                inner.image_id,
                inner.docker_image,
                inner.docker_entrypoint,
                inner.docker_args,
                inner.docker_env,
            )
            .await
        {
            Ok(_) => Ok(Response::new(CreateImageResponse {
                ack: Some(Ack {
                    success: true,
                    message: String::new(),
                }),
            })),
            Err(e) => Ok(Response::new(CreateImageResponse {
                ack: Some(Ack {
                    success: false,
                    message: e.to_string(),
                }),
            })),
        }
    }

    async fn destroy_image(
        &self,
        request: Request<DestroyImageRequest>,
    ) -> Result<Response<DestroyImageResponse>, Status> {
        let inner = request.into_inner();

        match self.image_service.delete_image(&inner.image_id).await {
            Ok(()) => Ok(Response::new(DestroyImageResponse {
                ack: Some(Ack {
                    success: true,
                    message: String::new(),
                }),
            })),
            Err(e) => Ok(Response::new(DestroyImageResponse {
                ack: Some(Ack {
                    success: false,
                    message: e.to_string(),
                }),
            })),
        }
    }

    async fn list_images(
        &self,
        _request: Request<ListImagesRequest>,
    ) -> Result<Response<ListImagesResponse>, Status> {
        match self.image_service.list_images().await {
            Ok(images) => {
                let proto_images = images.into_iter().map(|img| img.into()).collect();

                Ok(Response::new(ListImagesResponse {
                    ack: Some(Ack {
                        success: true,
                        message: String::new(),
                    }),
                    images: proto_images,
                }))
            }
            Err(e) => Ok(Response::new(ListImagesResponse {
                ack: Some(Ack {
                    success: false,
                    message: e.to_string(),
                }),
                images: vec![],
            })),
        }
    }

    async fn create_observation(
        &self,
        request: Request<CreateObservationRequest>,
    ) -> Result<Response<CreateObservationResponse>, Status> {
        let inner = request.into_inner();

        // Convert timestamps
        let start_time = inner
            .start_time
            .and_then(|ts| chrono::DateTime::from_timestamp(ts.seconds, ts.nanos as u32))
            .ok_or_else(|| Status::invalid_argument("Invalid start time"))?;

        let end_time = inner
            .end_time
            .and_then(|ts| chrono::DateTime::from_timestamp(ts.seconds, ts.nanos as u32))
            .ok_or_else(|| Status::invalid_argument("Invalid end time"))?;

        // If validate_only is true, just validate without creating
        if inner.validate_only {
            if let Err(e) = self
                .observation_service
                .validate_observation(
                    start_time,
                    end_time,
                    inner.number_of_replicants,
                    &inner.image_id,
                )
                .await
            {
                return Ok(Response::new(CreateObservationResponse {
                    ack: Some(Ack {
                        success: false,
                        message: e.to_string(),
                    }),
                    observation_id: None,
                }));
            }

            // Also validate that image exists
            if !self
                .image_service
                .validate_image_exists(&inner.image_id)
                .await
                .unwrap_or(false)
            {
                return Ok(Response::new(CreateObservationResponse {
                    ack: Some(Ack {
                        success: false,
                        message: format!("Image {} does not exist", inner.image_id),
                    }),
                    observation_id: None,
                }));
            }

            return Ok(Response::new(CreateObservationResponse {
                ack: Some(Ack {
                    success: true,
                    message: "Validation successful".to_string(),
                }),
                observation_id: None,
            }));
        }

        // Create the observation
        match self
            .observation_service
            .create_observation(
                start_time,
                end_time,
                inner.replicant_tags,
                inner.number_of_replicants,
                inner.image_id,
            )
            .await
        {
            Ok(observation) => Ok(Response::new(CreateObservationResponse {
                ack: Some(Ack {
                    success: true,
                    message: String::new(),
                }),
                observation_id: Some(observation.id),
            })),
            Err(e) => Ok(Response::new(CreateObservationResponse {
                ack: Some(Ack {
                    success: false,
                    message: e.to_string(),
                }),
                observation_id: None,
            })),
        }
    }

    async fn destroy_observation(
        &self,
        request: Request<DestroyObservationRequest>,
    ) -> Result<Response<DestroyObservationResponse>, Status> {
        let inner = request.into_inner();

        match self
            .observation_service
            .destroy_observation(&inner.observation_id)
            .await
        {
            Ok(()) => Ok(Response::new(DestroyObservationResponse {
                ack: Some(Ack {
                    success: true,
                    message: String::new(),
                }),
            })),
            Err(e) => Ok(Response::new(DestroyObservationResponse {
                ack: Some(Ack {
                    success: false,
                    message: e.to_string(),
                }),
            })),
        }
    }

    async fn list_observations(
        &self,
        _request: Request<ListObservationsRequest>,
    ) -> Result<Response<ListObservationsResponse>, Status> {
        match self.observation_service.find_all().await {
            Ok(observations) => {
                let proto_observations = observations.into_iter().map(|obs| obs.into()).collect();

                Ok(Response::new(ListObservationsResponse {
                    ack: Some(Ack {
                        success: true,
                        message: String::new(),
                    }),
                    observations: proto_observations,
                }))
            }
            Err(e) => Ok(Response::new(ListObservationsResponse {
                ack: Some(Ack {
                    success: false,
                    message: e.to_string(),
                }),
                observations: vec![],
            })),
        }
    }

    async fn list_replicants(
        &self,
        _request: Request<ListReplicantsRequest>,
    ) -> Result<Response<ListReplicantsResponse>, Status> {
        match self.replicant_service.find_all().await {
            Ok(replicants) => {
                let proto_replicants = replicants
                    .into_iter()
                    .map(|replicant| nexus_common::proto::generic::Replicant {
                        replicant_id: replicant.id.clone(),
                        version: replicant.version,
                        tags: replicant.tags,
                        status: nexus_common::proto::enums::Status::from(replicant.status) as i32,
                        hardware: Some(replicant.hardware.into()),
                        last_seen: Some(prost_types::Timestamp {
                            seconds: replicant.last_seen.timestamp(),
                            nanos: replicant.last_seen.timestamp_subsec_nanos() as i32,
                        }),
                        created_at: Some(prost_types::Timestamp {
                            seconds: replicant.created_at.timestamp(),
                            nanos: replicant.created_at.timestamp_subsec_nanos() as i32,
                        }),
                    })
                    .collect();

                Ok(Response::new(ListReplicantsResponse {
                    ack: Some(Ack {
                        success: true,
                        message: String::new(),
                    }),
                    replicants: proto_replicants,
                }))
            }
            Err(e) => Ok(Response::new(ListReplicantsResponse {
                ack: Some(Ack {
                    success: false,
                    message: e.to_string(),
                }),
                replicants: vec![],
            })),
        }
    }

    type SignallerStream = ReceiverStream<Result<NexusSignallerResponse, Status>>;

    async fn signaller(
        &self,
        request: Request<NexusSignallerRequest>,
    ) -> Result<Response<Self::SignallerStream>, Status> {
        let inner = request.into_inner();
        let version = inner.version;

        if version != "0.1.0" {
            // TODO: Implement version checking logic.
        }

        // Create channel for responses
        let (tx, rx) = mpsc::channel(32);

        let tx_clone = tx.clone();
        let replicant_service = self.replicant_service.clone();
        let observation_service = self.observation_service.clone();

        tokio::spawn(async move {
            let hardware_metrics = if let Ok(replicants) = replicant_service.find_all().await {
                replicants
                    .into_iter()
                    .flat_map(|r| {
                        let replicant_id = r.id.clone();
                        r.metrics.into_iter().map(move |m| {
                            let mut proto_metric: nexus_common::proto::generic::HardwareMetrics =
                                m.into();
                            proto_metric.replicant_id = replicant_id.clone();
                            proto_metric
                        })
                    })
                    .collect()
            } else {
                Vec::new()
            };

            let (observation_metrics, logs) = if let Ok(observations) =
                observation_service.find_all().await
            {
                let obs_metrics = observations
                    .iter()
                    .flat_map(|o| o.aggregated_metrics.clone())
                    .collect();

                let logs = observations
                    .iter()
                    .flat_map(|o| {
                        o.logs.iter().map(|log| {
                            let mut proto_log: nexus_common::proto::generic::LogEntry = log.into();
                            proto_log.observation_id = o.id.clone();
                            proto_log
                        })
                    })
                    .collect();

                (obs_metrics, logs)
            } else {
                (Vec::new(), Vec::new())
            };

            if !hardware_metrics.is_empty() || !observation_metrics.is_empty() || !logs.is_empty() {
                let _ = tx_clone
                    .send(Ok(NexusSignallerResponse {
                        timestamp: Some(create_timestamp()),
                        events: vec![],
                        logs,
                        observation_metrics,
                        hardware_metrics,
                    }))
                    .await;
            }
        });

        let mut event_rx = self.subscription_manager.subscribe();

        tokio::spawn(async move {
            use crate::transport::connections::SubscriptionEvent;

            let mut pending_events = Vec::new();
            let mut pending_logs = Vec::new();
            let mut pending_observation_metrics = Vec::new();
            let mut pending_hardware_metrics = Vec::new();

            loop {
                match event_rx.recv().await {
                    Ok(event) => {
                        match event {
                            SubscriptionEvent::Log(log) => {
                                pending_logs.push(log);
                            }
                            SubscriptionEvent::ObservationMetrics(m) => {
                                pending_observation_metrics.push(m);
                            }
                            SubscriptionEvent::HardwareMetrics(m) => {
                                pending_hardware_metrics.push(m);
                            }
                            SubscriptionEvent::ObservationCreated(obs) => {
                                pending_events.push(create_observation_event(
                                    obs,
                                    nexus_common::proto::enums::EventType::Create,
                                ));
                            }
                            SubscriptionEvent::ObservationUpdated(obs) => {
                                pending_events.push(create_observation_event(
                                    obs,
                                    nexus_common::proto::enums::EventType::Update,
                                ));
                            }
                            SubscriptionEvent::ObservationDeleted(id) => {
                                pending_events.push(create_observation_deleted_event(id));
                            }
                            SubscriptionEvent::ReplicantCreated(rep) => {
                                pending_events.push(create_replicant_event(
                                    rep,
                                    nexus_common::proto::enums::EventType::Create,
                                ));
                            }
                            SubscriptionEvent::ReplicantUpdated(rep) => {
                                pending_events.push(create_replicant_event(
                                    rep,
                                    nexus_common::proto::enums::EventType::Update,
                                ));
                            }
                            SubscriptionEvent::ReplicantDeleted(id) => {
                                pending_events.push(create_replicant_deleted_event(id));
                            }
                        }

                        if !pending_events.is_empty()
                            || !pending_logs.is_empty()
                            || !pending_observation_metrics.is_empty()
                            || !pending_hardware_metrics.is_empty()
                        {
                            let response = NexusSignallerResponse {
                                timestamp: Some(create_timestamp()),
                                events: std::mem::take(&mut pending_events),
                                logs: std::mem::take(&mut pending_logs),
                                observation_metrics: std::mem::take(
                                    &mut pending_observation_metrics,
                                ),
                                hardware_metrics: std::mem::take(&mut pending_hardware_metrics),
                            };

                            if tx.send(Ok(response)).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

fn create_timestamp() -> prost_types::Timestamp {
    let now = chrono::Utc::now();
    prost_types::Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}

fn replicant_to_proto(
    rep: crate::domain::entities::Replicant,
) -> nexus_common::proto::generic::Replicant {
    nexus_common::proto::generic::Replicant {
        replicant_id: rep.id.clone(),
        version: rep.version,
        tags: rep.tags,
        status: nexus_common::proto::enums::Status::from(rep.status) as i32,
        hardware: Some(rep.hardware.into()),
        last_seen: Some(prost_types::Timestamp {
            seconds: rep.last_seen.timestamp(),
            nanos: rep.last_seen.timestamp_subsec_nanos() as i32,
        }),
        created_at: Some(prost_types::Timestamp {
            seconds: rep.created_at.timestamp(),
            nanos: rep.created_at.timestamp_subsec_nanos() as i32,
        }),
    }
}

fn create_observation_event(
    obs: crate::domain::entities::Observation,
    event_type: nexus_common::proto::enums::EventType,
) -> nexus_common::proto::nexus::nexus_signaller_response::NexusSignallerEvent {
    use nexus_common::proto::nexus::nexus_signaller_response::nexus_signaller_event::Payload;

    nexus_common::proto::nexus::nexus_signaller_response::NexusSignallerEvent {
        r#type: event_type as i32,
        payload: Some(Payload::Observation(obs.into())),
    }
}

fn create_observation_deleted_event(
    id: String,
) -> nexus_common::proto::nexus::nexus_signaller_response::NexusSignallerEvent {
    use nexus_common::proto::nexus::nexus_signaller_response::nexus_signaller_event::Payload;

    nexus_common::proto::nexus::nexus_signaller_response::NexusSignallerEvent {
        r#type: nexus_common::proto::enums::EventType::Delete as i32,
        payload: Some(Payload::Observation(
            nexus_common::proto::generic::Observation {
                observation_id: id,
                ..Default::default()
            },
        )),
    }
}

fn create_replicant_event(
    rep: crate::domain::entities::Replicant,
    event_type: nexus_common::proto::enums::EventType,
) -> nexus_common::proto::nexus::nexus_signaller_response::NexusSignallerEvent {
    use nexus_common::proto::nexus::nexus_signaller_response::nexus_signaller_event::Payload;

    nexus_common::proto::nexus::nexus_signaller_response::NexusSignallerEvent {
        r#type: event_type as i32,
        payload: Some(Payload::Replicant(replicant_to_proto(rep))),
    }
}

fn create_replicant_deleted_event(
    id: String,
) -> nexus_common::proto::nexus::nexus_signaller_response::NexusSignallerEvent {
    use nexus_common::proto::nexus::nexus_signaller_response::nexus_signaller_event::Payload;

    nexus_common::proto::nexus::nexus_signaller_response::NexusSignallerEvent {
        r#type: nexus_common::proto::enums::EventType::Delete as i32,
        payload: Some(Payload::Replicant(
            nexus_common::proto::generic::Replicant {
                replicant_id: id,
                ..Default::default()
            },
        )),
    }
}
