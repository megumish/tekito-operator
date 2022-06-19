// 自動生成したい。

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
};

use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{Container, PodSpec, PodTemplateSpec},
    },
    apimachinery::pkg::apis::meta::v1::LabelSelector,
};
use kube::core::ObjectMeta;

fn main() -> anyhow::Result<()> {
    fs::create_dir_all("./target/config/managers")?;
    generate_manager()?;
    Ok(())
}

fn generate_manager() -> anyhow::Result<()> {
    let manager_name = format!("{}-manager", env!("CARGO_PKG_NAME"));
    let file_name = format!("{}-manager.yaml", env!("CARGO_PKG_NAME"));
    let mut file = File::create(format!("./target/config/managers/{}", file_name))?;
    let labels = {
        let mut labels = BTreeMap::new();
        labels.insert(
            "control-plane".to_string(),
            env!("CARGO_PKG_NAME").to_string(),
        );
        labels
    };
    let deployment = Deployment {
        metadata: ObjectMeta {
            name: Some(manager_name.clone()),
            namespace: option_env!("OPERATOR_NS").map(ToString::to_string),
            labels: Some(labels.clone()),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            selector: LabelSelector {
                match_labels: Some(labels.clone()),
                ..Default::default()
            },
            replicas: Some(1),
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    annotations: Some({
                        let mut annotations = BTreeMap::new();
                        annotations.insert(
                            "kubectl.kubernetes.io/default-container".to_string(),
                            "manager".to_string(),
                        );
                        annotations
                    }),
                    labels: Some(labels),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        command: Some(vec!["/manager".to_string()]),
                        image: Some(format!(
                            "quay.io/megumish/{}:v{}",
                            env!("CARGO_PKG_NAME"),
                            env!("CARGO_PKG_VERSION")
                        )),
                        ..Default::default()
                    }],
                    service_account_name: Some(manager_name.clone()),
                    ..Default::default()
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    };
    file.write_all(serde_yaml::to_string(&deployment)?.as_bytes())?;
    Ok(())
}
