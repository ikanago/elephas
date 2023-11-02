use actix_web::{get, web, HttpResponse};

use crate::model::nodeinfo::{
    NodeInfo, NodeInfoDiscovery, NodeInfoDiscoveryLink, NodeInfoMetadata, NodeInfoSoftware,
};

#[get("/.well-known/nodeinfo")]
#[tracing::instrument()]
pub async fn nodeinfo_discovery(host_name: web::Data<String>) -> HttpResponse {
    let nodeinfo_discovery = NodeInfoDiscovery {
        links: vec![NodeInfoDiscoveryLink {
            rel: "http://nodeinfo.diaspora.software/ns/schema/2.1".to_string(),
            href: format!("https://{}/nodeinfo/2.1", host_name.into_inner()),
        }],
    };
    HttpResponse::Ok().json(nodeinfo_discovery)
}

#[get("/nodeinfo/2.1")]
#[tracing::instrument()]
pub async fn nodeinfo(host_name: web::Data<String>) -> HttpResponse {
    let nodeinfo = NodeInfo {
        version: "2.1".to_string(),
        open_registrations: false,
        software: NodeInfoSoftware {
            name: "elephas".to_string(),
            version: "0.1.0".to_string(),
        },
        metadata: NodeInfoMetadata {
            node_description: "elephas is a simple ActivityPub server.".to_string(),
        },
    };
    HttpResponse::Ok().json(nodeinfo)
}
