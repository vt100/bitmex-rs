use bitmex::models::GetLiquidationRequest;
use bitmex::BitMEX;
use failure::Fallible;
use std::env::var;
use tokio::runtime::Runtime;

#[test]
fn get_liquidation() -> Fallible<()> {
    ::dotenv::dotenv().ok();
    let mut rt = Runtime::new()?;

    let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
    let fut = bm.get_liquidation(GetLiquidationRequest {
        ..Default::default()
    })?;

    let _ = rt.block_on(fut)?;
    Ok(())
}
