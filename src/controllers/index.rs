use crate::config::{self};
use crate::service::upload::upload_files;
use actix_web::HttpResponse;
use actix_multipart::Multipart;

pub struct Index {}

impl Index {

    /// 上传文件: 图片
    pub async fn upload_images(payload: Multipart) -> HttpResponse {
       upload_files(&config::UPLOAD_IMAGE_TYPES, payload).await
    }

}

