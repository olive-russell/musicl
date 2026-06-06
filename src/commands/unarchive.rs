use anyhow::{Result};

use crate::ctx::Ctx;

pub fn handle(ctx: &mut Ctx, path: std::path::PathBuf) -> Result<()> {
    println!("Unarchiving track: {:?} to {:?}", path, ctx.status_path);
    Ok(())
}
