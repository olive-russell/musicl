use anyhow::Result;

use crate::ctx::Ctx;

pub fn handle(ctx: &mut Ctx) -> Result<()> {
    let _ = ctx;
    println!("Cleaning database...");
    Ok(())
}
