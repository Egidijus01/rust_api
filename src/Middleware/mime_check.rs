
use warp::reject::Reject;
use warp::Filter;





#[derive(Debug)]
struct InvalidContentType;

impl Reject for InvalidContentType {}




pub fn check_content_type() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::header::headers_cloned()
        .and_then(|headers: warp::http::HeaderMap| async move {
            if let Some(content_type) = headers.get("content-type") {
                if let Ok(content_type) = content_type.to_str() {
                    if content_type.to_lowercase() == "application/json"

                    {
                        return Ok(());
                    }
                }
            }

            println!("Invalid or missing Content-Type header");
            Err(warp::reject::custom(InvalidContentType))
        })
        .untuple_one()
}



pub fn check_image_format(data: &[u8]) -> Option<&str> {
    if data.len() >= 8 {
        let png_signature: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
        let jpeg_signature: [u8; 3] = [255, 216, 255];

        if data.starts_with(&png_signature) {
            return Some("PNG");
        } else if data[0..3].eq(&jpeg_signature) {
            return Some("JPEG");
        }
    }

    None
}

pub fn check_image_size(data: &[u8]) -> Option<usize> {
    let size = data.len();
    let max_size_bytes = 500*1024; //500kB
    if size <= max_size_bytes {
        Some(size)
    } else {
        None
    }
}

pub fn check_file_size(data: &[u8]) -> Option<usize> {
    let size = data.len();
    let max_size_bytes = 10000*1024;  // 10 MB
    if size <= max_size_bytes {
        Some(size)
    } else {
        None
    }
}
pub fn check_file_format(data: &[u8]) -> Option<&str> {
    if data.len() >= 8 {
        let pdf_signature: [u8; 4] = [37, 80, 68, 70];
        let txt_signature: [u8; 4] = [84, 69, 88, 84];
        let docx_signature: [u8; 4] = [80, 75, 3, 4];
        let doc_signature: [u8; 4] = [208, 207, 17, 224];

        if data.starts_with(&pdf_signature) {
            return Some("PDF");
        } else if data.starts_with(&txt_signature) {
            return Some("TXT");
        } else if data.starts_with(&docx_signature) {
            return Some("DOCX");
        } else if data.starts_with(&doc_signature) {
            return Some("DOC");
        }
    }

    None
}



