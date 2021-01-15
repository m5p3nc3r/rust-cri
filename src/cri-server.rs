use tonic::{transport::Server, Request, Response, Status};

use cri::runtime_service_server::{RuntimeService, RuntimeServiceServer};
use cri::{VersionRequest, VersionResponse};
use cri::{RunPodSandboxRequest, RunPodSandboxResponse};
use cri::{StopPodSandboxRequest, StopPodSandboxResponse};
use cri::{RemovePodSandboxRequest, RemovePodSandboxResponse};
use cri::{PodSandboxStatusRequest, PodSandboxStatusResponse};
use cri::{ListPodSandboxRequest, ListPodSandboxResponse};
use cri::{CreateContainerRequest, CreateContainerResponse};
use cri::{StartContainerRequest, StartContainerResponse};
use cri::{StopContainerRequest, StopContainerResponse};
use cri::{RemoveContainerRequest, RemoveContainerResponse};
use cri::{ListContainersRequest, ListContainersResponse};
use cri::{ContainerStatusRequest, ContainerStatusResponse};
use cri::{UpdateContainerResourcesRequest, UpdateContainerResourcesResponse};
use cri::{ReopenContainerLogRequest, ReopenContainerLogResponse};
use cri::{ExecSyncRequest, ExecSyncResponse};
use cri::{ExecRequest, ExecResponse};
use cri::{AttachRequest, AttachResponse};
use cri::{PortForwardRequest, PortForwardResponse};
use cri::{ContainerStatsRequest, ContainerStatsResponse};
use cri::{ListContainerStatsRequest, ListContainerStatsResponse};
use cri::{UpdateRuntimeConfigRequest, UpdateRuntimeConfigResponse};
use cri::{StatusRequest, StatusResponse};

pub mod cri {
    tonic::include_proto!("runtime.v1"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyRuntimeService {}

//  // Version of the kubelet runtime API.
//  string version = 1;
//  // Name of the container runtime.
//  string runtime_name = 2;
//  // Version of the container runtime. The string must be
//  // semver-compatible.
//  string runtime_version = 3;
//  // API version of the container runtime. The string must be
//  // semver-compatible.
//  string runtime_api_version = 4;

#[tonic::async_trait]
impl RuntimeService for MyRuntimeService {

    async fn version( 
        &self,
        request: Request<VersionRequest>,
    ) -> Result<Response<VersionResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = cri::VersionResponse {
            version: "version".to_string(),
            runtime_name: "runtime".to_string(),
            runtime_version: "runtime version".to_string(),
            runtime_api_version: "runtime api version".to_string()
        };

        Ok(Response::new(reply))
    }

    async fn run_pod_sandbox(
        &self,
        request: Request<RunPodSandboxRequest>
    ) -> Result<Response<RunPodSandboxResponse>, Status> {
        println!("Got a request: {:?}", request);

        Err(Status::unimplemented("RunPodSandbox"))
    }

    async fn stop_pod_sandbox(
        &self,
        request: Request<StopPodSandboxRequest>
    ) -> Result<Response<StopPodSandboxResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("StopPodSandbox"))
    }

    async fn remove_pod_sandbox(
        &self,
        request: Request<RemovePodSandboxRequest>
    ) -> Result<Response<RemovePodSandboxResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("RemovePodSandbox"))
    }
    
    async fn pod_sandbox_status(
        &self,
        request: Request<PodSandboxStatusRequest>
    ) -> Result<Response<PodSandboxStatusResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("PodSandboxStatus"))
    }

    async fn list_pod_sandbox(
        &self,
        request: Request<ListPodSandboxRequest>
    ) -> Result<Response<ListPodSandboxResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("ListPodSandbox"))
    }

    async fn create_container(
        &self,
        request: Request<CreateContainerRequest>
    ) -> Result<Response<CreateContainerResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("CreateContainer"))
    }

    async fn start_container(
        &self,
        request: Request<StartContainerRequest>
    ) -> Result<Response<StartContainerResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("StartContainer"))
    }

    async fn stop_container(
        &self,
        request: Request<StopContainerRequest>
    ) -> Result<Response<StopContainerResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("StopContainer"))
    }

    async fn remove_container(
        &self,
        request: Request<RemoveContainerRequest>
    ) -> Result<Response<RemoveContainerResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("RemoveContainer"))
    }

    async fn list_containers(
        &self,
        request: Request<ListContainersRequest>
    ) -> Result<Response<ListContainersResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("ListContainers"))
    }

    async fn container_status(
        &self,
        request: Request<ContainerStatusRequest>
    ) -> Result<Response<ContainerStatusResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("ContainerStatus"))
    }

    async fn update_container_resources(
        &self,
        request: Request<UpdateContainerResourcesRequest>
    ) -> Result<Response<UpdateContainerResourcesResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("UpdateContainerResources"))
    }

    async fn reopen_container_log(
        &self,
        request: Request<ReopenContainerLogRequest>
    ) -> Result<Response<ReopenContainerLogResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("ReopenContainerLog"))
    }

    async fn exec_sync(
        &self,
        request: Request<ExecSyncRequest>
    ) -> Result<Response<ExecSyncResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("ExecSync"))
    }

    async fn exec(
        &self,
        request: Request<ExecRequest>
    ) -> Result<Response<ExecResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("Exec"))
    }

    async fn attach(
        &self,
        request: Request<AttachRequest>
    ) -> Result<Response<AttachResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("Attach"))
    }

    async fn port_forward(
        &self,
        request: Request<PortForwardRequest>
    ) -> Result<Response<PortForwardResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("PortForward"))
    }

    async fn container_stats(
        &self,
        request: Request<ContainerStatsRequest>
    ) -> Result<Response<ContainerStatsResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("ContainerStats"))
    }

    async fn list_container_stats(
        &self,
        request: Request<ListContainerStatsRequest>
    ) -> Result<Response<ListContainerStatsResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("ListContainerStats"))
    }

    async fn update_runtime_config(
        &self,
        request: Request<UpdateRuntimeConfigRequest>
    ) -> Result<Response<UpdateRuntimeConfigResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("UpdateRuntimeConfig"))
    }

    async fn status(
        &self,
        request: Request<StatusRequest>
    ) -> Result<Response<StatusResponse>, Status> {
        println!("Got a request: {:?}", request);
        Err(Status::unimplemented("Status"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let runtime_service = MyRuntimeService::default();

    Server::builder()
        .add_service(RuntimeServiceServer::new(runtime_service))
        .serve(addr)
        .await?;

    Ok(())
}