use lazy_static::*;
use std::collections::HashMap;

struct Resource {
    content_type: &'static str,
    content: &'static [u8],
}

lazy_static! {
    static ref PAGES: HashMap<&'static str, Resource> = {
        let mut m = HashMap::new();
        m.insert(
            "app.css",
            Resource {
                content_type: "text/css",
                content: include_bytes!("../frontend/app.css"),
            },
        );
        m.insert(
            "app.js",
            Resource {
                content_type: "text/javascript",
                content: include_bytes!("../frontend/app.js"),
            },
        );
        m.insert(
            "icons/buttons-72.png",
            Resource {
                content_type: "image/png",
                content: include_bytes!("../frontend/icons/buttons-72.png"),
            },
        );
        m.insert(
            "icons/buttons-96.png",
            Resource {
                content_type: "image/png",
                content: include_bytes!("../frontend/icons/buttons-96.png"),
            },
        );
        m.insert(
            "icons/buttons-128.png",
            Resource {
                content_type: "image/png",
                content: include_bytes!("../frontend/icons/buttons-128.png"),
            },
        );
        m.insert(
            "icons/buttons-144.png",
            Resource {
                content_type: "image/png",
                content: include_bytes!("../frontend/icons/buttons-144.png"),
            },
        );
        m.insert(
            "icons/buttons-152.png",
            Resource {
                content_type: "image/png",
                content: include_bytes!("../frontend/icons/buttons-152.png"),
            },
        );
        m.insert(
            "icons/buttons-192.png",
            Resource {
                content_type: "image/png",
                content: include_bytes!("../frontend/icons/buttons-192.png"),
            },
        );
        m
    };
}

pub async fn handle(
    tail: warp::filters::path::Tail,
) -> Result<impl warp::reply::Reply, warp::reject::Rejection> {
    PAGES
        .get(tail.as_str())
        .ok_or_else(warp::reject::not_found)
        .map(|page| warp::reply::with_header(page.content, "Content-Type", page.content_type))
}
