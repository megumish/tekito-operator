// 自動生成したい。

use std::{
    fs::{self, File},
    io::Write,
};

use k8s_openapi::api::core::v1::Namespace;
use kube::core::ObjectMeta;

fn main() -> anyhow::Result<()> {
    fs::create_dir_all("./target/config/namespaces")?;
    generate_ns()?;
    Ok(())
}

fn generate_ns() -> anyhow::Result<()> {
    let file_name = format!("ns.yaml",);
    let mut file = File::create(format!("./target/config/namespaces/{}", file_name))?;
    let ns = Namespace {
        metadata: ObjectMeta {
            name: option_env!("OPERATOR_NS").map(ToString::to_string),
            ..Default::default()
        },
        ..Default::default()
    };
    file.write_all(serde_yaml::to_string(&ns)?.as_bytes())?;
    Ok(())
}
