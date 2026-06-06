use anyhow::Result;
use std::path::PathBuf;

use crate::ctx::Ctx;

pub fn handle(ctx: &mut Ctx, path: PathBuf) -> Result<()> {
    println!("Adding: {:?} to {:?}", path, ctx.status_path);
    Ok(())
}
