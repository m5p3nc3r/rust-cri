use tonic::{Request, Response, Status};

use cri::image_service_server::{ImageService, ImageServiceServer};

use cri::{ListImagesRequest, ListImagesResponse};
use cri::{ImageStatusRequest, ImageStatusResponse};
use cri::{PullImageRequest, PullImageResponse};
use cri::{RemoveImageRequest, RemoveImageResponse};
use cri::{ImageFsInfoRequest, ImageFsInfoResponse};


pub mod cri {
    tonic::include_proto!("runtime.v1"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyImageService {}    

#[tonic::async_trait]
impl ImageService for MyImageService {
    async fn list_images(
        &self,
        request: Request<ListImagesRequest>
    ) -> Result<Response<ListImagesResponse>, Status> {
        println!("Got a request: {:?}", request);

        Err(Status::unimplemented("ListImages"))
    }

    async fn image_status(
        &self,
        request: Request<ImageStatusRequest>
    ) -> Result<Response<ImageStatusResponse>, Status> {
        println!("Got a request: {:?}", request);

        Err(Status::unimplemented("ImageStatus"))
    }

    async fn pull_image(
        &self,
        request: Request<PullImageRequest>
    ) -> Result<Response<PullImageResponse>, Status> {
        println!("Got a request: {:?}", request);

        Err(Status::unimplemented("PullImage"))
    }

    async fn remove_image(
        &self,
        request: Request<RemoveImageRequest>
    ) -> Result<Response<RemoveImageResponse>, Status> {
        println!("Got a request: {:?}", request);

        Err(Status::unimplemented("RemoveImages"))
    }

    async fn image_fs_info(
        &self,
        request: Request<ImageFsInfoRequest>
    ) -> Result<Response<ImageFsInfoResponse>, Status> {
        println!("Got a request: {:?}", request);

        Err(Status::unimplemented("ImageFsInfo"))
    }   
}

pub fn new_image_service_server() -> ImageServiceServer<MyImageService>  {
    return ImageServiceServer::new(MyImageService::default())
}

