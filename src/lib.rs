use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[kube(group = "megumi.sh", version = "v1alpha1", kind = "Tekito", namespaced)]
pub struct TekitoSpec {
    pub neko: String,
}
