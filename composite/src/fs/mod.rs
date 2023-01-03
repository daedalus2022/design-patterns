pub mod file;
pub mod folder;

pub trait Component{
    fn search(&self, keyword: &str);
}