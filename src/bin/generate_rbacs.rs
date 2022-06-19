// 自動生成したい。

use std::{
    fs::{self, File},
    io::Write,
};

use k8s_openapi::api::{
    core::v1::ServiceAccount,
    rbac::v1::{PolicyRule, Role, RoleBinding, RoleRef, Subject},
};
use kube::core::ObjectMeta;

fn main() -> anyhow::Result<()> {
    fs::create_dir_all("./target/config/rbacs")?;
    generate_service_account()?;
    generate_role()?;
    generate_role_binding()?;
    Ok(())
}

fn generate_service_account() -> anyhow::Result<()> {
    let manager_name = format!("{}-manager", env!("CARGO_PKG_NAME"));
    let file_name = format!("{}-manager-sa.yaml", env!("CARGO_PKG_NAME"));
    let mut file = File::create(format!("./target/config/rbacs/{}", file_name))?;
    let sa = ServiceAccount {
        metadata: ObjectMeta {
            name: Some(manager_name),
            namespace: option_env!("OPERATOR_NS").map(ToString::to_string),
            ..Default::default()
        },
        ..Default::default()
    };
    file.write_all(serde_yaml::to_string(&sa)?.as_bytes())?;
    Ok(())
}

fn generate_role() -> anyhow::Result<()> {
    let manager_name = format!("{}-manager", env!("CARGO_PKG_NAME"));
    let file_name = format!("{}-manager-role.yaml", env!("CARGO_PKG_NAME"));
    let mut file = File::create(format!("./target/config/rbacs/{}", file_name))?;
    let role = Role {
        metadata: ObjectMeta {
            name: Some(format!("{}-role", manager_name)),
            namespace: option_env!("OPERATOR_NS").map(ToString::to_string),
            ..Default::default()
        },
        rules: Some(vec![
            PolicyRule {
                api_groups: Some(vec!["megumi.sh".to_string()]),
                resources: Some(vec!["*".to_string()]),
                verbs: vec!["*".to_string()],
                ..Default::default()
            },
            PolicyRule {
                api_groups: Some(vec!["".to_string()]),
                resources: Some(vec!["ConfigMap".to_string()]),
                verbs: vec!["*".to_string()],
                ..Default::default()
            },
        ]),
    };
    file.write_all(serde_yaml::to_string(&role)?.as_bytes())?;
    Ok(())
}

fn generate_role_binding() -> anyhow::Result<()> {
    let manager_name = format!("{}-manager", env!("CARGO_PKG_NAME"));
    let file_name = format!("{}-manager-role-binding.yaml", env!("CARGO_PKG_NAME"));
    let mut file = File::create(format!("./target/config/rbacs/{}", file_name))?;
    let role_binding = RoleBinding {
        metadata: ObjectMeta {
            name: Some(format!("{}-role-binding", manager_name)),
            namespace: option_env!("OPERATOR_NS").map(ToString::to_string),
            ..Default::default()
        },
        role_ref: RoleRef {
            api_group: "rbac.authorization.k8s.io".to_string(),
            kind: "Role".to_string(),
            name: format!("{}-role", manager_name),
        },
        subjects: Some(vec![Subject {
            kind: "ServiceAccount".to_string(),
            name: manager_name,
            namespace: option_env!("OPERATOR_NS").map(ToString::to_string),
            ..Default::default()
        }]),
    };
    file.write_all(serde_yaml::to_string(&role_binding)?.as_bytes())?;
    Ok(())
}
