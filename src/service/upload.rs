
use fluffy::{datetime, random};
use std::fs;
use actix_web::{HttpResponse, web};
use std::path::Path;
use crate::models::{UploadResult};
use actix_multipart::Multipart;
use futures::StreamExt;
use std::io::Write;

// 上传文件
pub async fn upload_files(file_types: &[&str],  mut payload: Multipart) -> HttpResponse { 
    let upload_result = |code: u32, message: &str, path: &str| { 
        let result = UploadResult{code: code as usize, message, path};
        HttpResponse::Ok().json(result)
    };
    let upload_error = |code: u32, message: &str| {  //上传成功返回
        upload_result(code, message, "")
    };
    let upload_success = |path: &str| {  //上传失败返回
        upload_result(0, "", path)
    };
    while let Some(item) = payload.next().await { 
        let mut field = if let Ok(v) = item { v } else { return upload_error(401, "获取上传文件失败"); };

        // 检测文件类型, 只能上传图片类型
        let mime = field.content_type();
        let file_type = mime.type_().as_str();
        let file_ext = mime.subtype().as_str();
        let file_mine = format!("{}/{}", file_type, file_ext);
        if !file_types.contains(&file_mine.as_str()) { 
            return upload_error(4011, "上传的不是合法的图片文件");
        }

        //处理文件按日期目录进行存放
        let save_path = Path::new("./public/upload");
        if !save_path.is_dir() { 
            return upload_error(4012, "上传文件目录不存在");
        }
        let (year, month_, day_) = datetime::date();
        let month = if month_ > 9 { month_.to_string() } else { format!("0{}", month_) }; //前面补零: 月
        let day = if day_ > 9 { day_.to_string() } else { format!("0{}", day_) }; //前面补零: 日
        // 对目录(年)进行判断
        let save_year = format!("{}/{}", save_path.display(), year);
        let save_year_path = Path::new(&save_year);
        if !save_year_path.is_dir() { 
            if let Err(_) = fs::create_dir(&save_year_path) { 
                return upload_error(line!(), "创建目录(year)失败");
            }
        }
        // 对目录(月)进行判断
        let save_month = format!("{}/{}", save_year_path.display(), month);
        let save_month_path = Path::new(&save_month);
        if !save_month_path.is_dir() { 
            if let Err(_) = fs::create_dir(&save_month_path) { 
                return upload_error(line!(), "创建目录(month)失败");
            }
        }
        // 对目录(日)进行判断
        let save_day = format!("{}/{}", save_month_path.display(), day);
        let save_day_path = Path::new(&save_day);
        if !save_day_path.is_dir() { 
            if let Err(_) = fs::create_dir(&save_day_path) { 
                return upload_error(line!(), "创建目录(day)失败");
            }
        }
        let save_file_name = format!("{}.{}", random::rand_str(16), file_ext); //要保存的文件鋁名称
        let save_file_path = format!("{}/{}", save_day_path.display(), save_file_name); //保存的文件路径

        //let content_type = if let Some(v) = field.content_disposition() { v } else { return upload_error(402, "获取上传文件信息错误"); };
        //let file_name = if let Some(v) = content_type.get_filename() { v } else { return upload_error(403, "获取上传文件名称失败"); };
        let file_url = format!("/upload/{}/{}/{}/{}", year, month, day, save_file_name);
        //let file_path = dbg!(format!("./public/upload/{}", file_name));
        let mut f = if let Ok(v) = web::block(|| std::fs::File::create(save_file_path)).await { v } else { return upload_error(405, "创建临时文件失败"); };
        while let Some(chunk) = field.next().await { 
            let data = if let Ok(v) = chunk { v } else { return upload_error(406, "读取文件信息失败"); };
            f = if let Ok(v) = web::block(move || f.write_all(&data).map(|_| f)).await { v } else { return upload_error(408, "保存文件信息失败"); };
        }
        return upload_success(&file_url);
    }

    upload_error(4409, "上传文件失败")
}