use anyhow::{Result};

use crate::ctx::Ctx;

pub fn handle(ctx: &mut Ctx, path: std::path::PathBuf) -> Result<()> {
    let _ = ctx;
    println!("Removing track: {:?}", path);
    Ok(())
}
