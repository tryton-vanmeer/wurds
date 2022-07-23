use anyhow::Result;
use vergen::{vergen, Config};
use vergen::{ShaKind, TimestampKind};

fn main() -> Result<()> {
    let mut vergen_config = Config::default();
    *vergen_config.git_mut().sha_kind_mut() = ShaKind::Short;
    *vergen_config.git_mut().commit_timestamp_kind_mut() = TimestampKind::DateOnly;

    vergen(vergen_config)
}
