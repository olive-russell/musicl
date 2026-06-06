use anyhow::Result;

use crate::ctx::Ctx;

pub fn handle(ctx: &mut Ctx) -> Result<()> {
    println!("Checking status at: {:?}", ctx.status_path);
    Ok(())
}
