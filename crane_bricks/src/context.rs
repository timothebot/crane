
pub struct ActionContext {
    pub dry_run: bool,
}

impl ActionContext {
    pub fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }
}
