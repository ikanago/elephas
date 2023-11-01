use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct NodeInfoDiscoveryLink {
    pub rel: String,
    pub href: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeInfoDiscovery {
    pub links: Vec<NodeInfoDiscoveryLink>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeInfoSoftware {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeInfoMetadata {
    pub node_description: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeInfo {
    pub version: String,
    pub open_registrations: bool,
    pub software: NodeInfoSoftware,
    pub metadata: NodeInfoMetadata,
}
