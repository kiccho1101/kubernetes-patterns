use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Ok from version 1.0.0")
}

pub async fn healthz() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn test_index_ok() {
        let resp = index().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_healthz_ok() {
        let resp = healthz().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
