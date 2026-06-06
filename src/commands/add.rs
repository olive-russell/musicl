use anyhow::Result;
use std::path::PathBuf;

use crate::ctx::Ctx;

pub fn handle(ctx: &mut Ctx, path: PathBuf) -> Result<()> {
    let _ = ctx;
    _ = ctx.connection;
    println!("Adding: {:?}", path);
    Ok(())
}
