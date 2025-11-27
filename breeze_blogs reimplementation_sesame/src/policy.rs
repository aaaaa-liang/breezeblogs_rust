use sesame::policy::{PrivacyRegion, NoPolicy};

#[derive(Clone)]
pub struct YouContext;

impl PrivacyRegion for YouContext {
    type Policy = NoPolicy;  // we use NoPolicy for baseline

    fn new() -> Self { YouContext }
}
