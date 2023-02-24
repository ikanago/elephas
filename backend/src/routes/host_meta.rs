use actix_web::{get, web, HttpResponse, Responder};

#[get("/.well-known/host-meta")]
pub async fn host_meta(host_name: web::Data<String>) -> impl Responder {
    let host_name = &**host_name;
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/xrd+xml; charset=utf-8"))
        .body(format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
    <Link rel="lrdd" type="application/xrd+xml" template="https://${host_name}/.well-known/webfinger?resource={{uri}}"/>
</XRD>"#))
}
