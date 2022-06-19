// 自動生成したい。

use std::{
    fs::{self, File},
    io::Write,
};

use kube::CustomResourceExt;
use tekito_operator::Tekito;

fn main() -> anyhow::Result<()> {
    fs::create_dir_all("./target/config/crds")?;
    generate_crd::<Tekito>()?;
    Ok(())
}

fn generate_crd<Crd: CustomResourceExt>() -> anyhow::Result<()> {
    let file_name = format!("{}.yaml", Crd::crd_name());
    let mut file = File::create(format!("./target/config/crds/{}", file_name))?;
    file.write_all(serde_yaml::to_string(&Crd::crd())?.as_bytes())?;
    Ok(())
}
