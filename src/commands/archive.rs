use anyhow::Result;

use crate::ctx::Ctx;

pub fn handle(ctx: &mut Ctx, path: std::path::PathBuf) -> Result<()> {
    let _ = ctx;
    println!("Archiving track: {:?}", path);
    Ok(())
}
