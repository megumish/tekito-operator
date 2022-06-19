use std::{collections::BTreeMap, io::BufRead, sync::Arc};

use futures::StreamExt;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{
    api::{ListParams, Patch, PatchParams},
    core::ObjectMeta,
    runtime::{controller::Action, Controller},
    Api, Client, Config, Resource,
};
use tekito_operator::Tekito;
use thiserror::Error;
use tokio::time::Duration;
use tracing::{info, warn};

#[derive(Debug, Error)]
enum Error {
    #[error("Failed to create ConfigMap: {0}")]
    ConfigMapCreationFailed(#[source] kube::Error),
    #[error("MissingObjectKey: {0}")]
    MissingObjectKey(&'static str),
}

async fn reconcile(tekito: Arc<Tekito>, ctx: Arc<Data>) -> Result<Action, Error> {
    info!("Reconciling Tekito: {:?}", tekito.metadata.name);
    let client = &ctx.client;

    let mut contents = BTreeMap::new();
    contents.insert("neko".to_string(), tekito.spec.neko.clone());
    let owner_reference = tekito.controller_owner_ref(&()).unwrap();
    let cm = ConfigMap {
        metadata: ObjectMeta {
            name: tekito.metadata.name.clone(),
            owner_references: Some(vec![owner_reference]),
            ..ObjectMeta::default()
        },
        data: Some(contents),
        ..Default::default()
    };
    let cm_api = Api::<ConfigMap>::namespaced(
        client.clone(),
        tekito
            .metadata
            .namespace
            .as_ref()
            .ok_or(Error::MissingObjectKey(".metadata.namespace"))?,
    );
    cm_api
        .patch(
            cm.metadata
                .name
                .as_ref()
                .ok_or(Error::MissingObjectKey(".metadata.name"))?,
            &PatchParams::apply("tekito.kube-rt.megumi.sh"),
            &Patch::Apply(&cm),
        )
        .await
        .map_err(Error::ConfigMapCreationFailed)?;

    Ok(Action::requeue(Duration::from_secs(300)))
}

fn error_policy(error: &Error, _ctx: Arc<Data>) -> Action {
    info!("Error: {:?}", error);
    Action::requeue(Duration::from_secs(1))
}

struct Data {
    client: Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::infer().await;
    info!("{config:?}");

    let client = Client::try_default().await?;

    let tekitos = Api::<Tekito>::all(client.clone());
    let cms = Api::<ConfigMap>::all(client.clone());

    info!("Starting controller");
    info!("press <enter> to force a reconiliation of all objects");

    let (mut reload_tx, reload_rx) = futures::channel::mpsc::channel(0);
    // Using a regular background thread since tokio::io::stdin() doesn't allow aborting reads,
    // and its worker prevents the Tokio runtime from shutting down.
    std::thread::spawn(move || {
        for _ in std::io::BufReader::new(std::io::stdin()).lines() {
            let _ = reload_tx.try_send(());
        }
    });

    Controller::new(tekitos, ListParams::default())
        .owns(cms, ListParams::default())
        .reconcile_all_on(reload_rx.map(|_| ()))
        .shutdown_on_signal()
        .run(reconcile, error_policy, Arc::new(Data { client }))
        .for_each(|res| async move {
            match res {
                Ok(o) => info!("reconciled {:?}", o),
                Err(e) => warn!("reconcile failed: {}", e),
            }
        })
        .await;
    info!("controller terminated");
    Ok(())
}
