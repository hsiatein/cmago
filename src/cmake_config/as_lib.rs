

pub trait AsLib{
    fn get_path<'a>(&'a self)->&'a String;
}