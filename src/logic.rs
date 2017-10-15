use ::Config;

pub struct Logic<'a> {
    config: &'a Config
}

impl<'a> Logic<'a> {
    pub fn new(config: &Config) -> Logic {
        Logic { config: config }
    }
}
