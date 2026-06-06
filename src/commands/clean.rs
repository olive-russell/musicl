use anyhow::Result;

use crate::ctx::Ctx;

pub fn handle(ctx: &mut Ctx) -> Result<()> {
    println!("Cleaning database at: {:?}", ctx.status_path);
    Ok(())
}
