use crate::resource::Resource;
pub struct Collector {
    pub tickrate: u64,
    pub output: Vec<Resource>,
    pub consumption: Vec<Resource>,
}
pub struct Converter {
    pub tickrate: u64,
    pub output: Vec<Resource>,
    pub input: Vec<Resource>,
}

pub struct Generator {
    pub tickrate: u64,
    pub output: Vec<Resource>,
    pub input: Vec<Resource>,
}
