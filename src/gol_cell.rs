#[derive(Clone)]
pub struct GolCell {
    status: u8,
}

impl GolCell {
    pub fn new() -> GolCell {
        GolCell { status: 0 }
    }
}
