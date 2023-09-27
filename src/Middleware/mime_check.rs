
use warp::reject::Reject;
use warp::Filter;










#[derive(Debug)]
struct InvalidContentType;

impl Reject for InvalidContentType {}





pub fn check_content_type() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("content-type")
        .and_then(|content_type: Option<String>| async move {
            match content_type {
                Some(ct) if ct.to_lowercase() == "application/json" => Ok(()),
                Some(_) => Err(warp::reject::custom(InvalidContentType)),
                None => {
                    println!("Received request with no Content-Type");
                    Ok(())
                }
            }
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
    let max_size_bytes = 500*1024;
    if size <= max_size_bytes {
        Some(size)
    } else {
        None
    }
}




