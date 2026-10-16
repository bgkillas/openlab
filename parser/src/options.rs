use crate::units::{AngelUnit, TimeUnit};
use std::path::Path;
#[derive(Clone, Copy)]
pub struct ParserOptions<'a> {
    pub path: Option<&'a Path>,
    pub angel: AngelUnit,
    pub time: TimeUnit,
}
impl<'a> ParserOptions<'a> {
    pub fn new(path: Option<&'a Path>) -> Self {
        Self {
            path,
            angel: AngelUnit::default(),
            time: TimeUnit::default(),
        }
    }
}
