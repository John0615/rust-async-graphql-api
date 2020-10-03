// use serde::ser::Serialize;
use serde_derive::{Serialize};


#[derive(Serialize)]
pub struct UploadResult<'a> { 
    pub code: usize, //错误代码, 0:表示成功
    pub message: &'a str, //错误信息
    pub path: &'a str, //上传的文件的路径
}

#[derive(Serialize)]
pub struct OSSData<'a> { 
    pub access_id: &'a str,
    pub host: &'a str,
    pub policy: &'a str,
    pub signature: &'a str,
    pub expire: u64,
}

/// oss返回的地址
#[derive(Serialize)]
pub struct OSSResult<'a> { 
    pub code: usize,
    pub success: bool,
    pub msg: &'static str,
    pub data: OSSData<'a>,
}

#[macro_export]
macro_rules! get_fields {
    ($struct: ident, [$($field: ident => $type: ident,)+]) => {
        
        /// 得到所有列表字段
        fn get_fields() -> &'static str { 
            concat!("id", $(",", stringify!($field)),+)
        }
    
        /// 得到单条记录
        fn get_record(r: DbRow) -> Self { 
            let mut row = Self::default();
            let (id, $($field),+): (usize, $($type),+) = from_row!(r);
            row.id = id;
            $(row.$field = $field;)+
            row
        }
    }
}
