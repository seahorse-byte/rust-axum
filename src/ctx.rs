#[derive(Clone, Debug)]
pub struct Ctx {
    pub user_id: u64,
    // pub user_name: String,
}

// Constructor
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Property accessors
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
