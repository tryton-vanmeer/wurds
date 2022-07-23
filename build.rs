use anyhow::Result;
use vergen::{vergen, Config, ShaKind};

fn main() -> Result<()> {
    let mut vergen_config = Config::default();
    *vergen_config.git_mut().sha_kind_mut() = ShaKind::Short;

    vergen(vergen_config)
}
