#[ic_cdk_macros::query]
fn healthcheck() -> String {
    format!("RUST IC setup properly")
}
