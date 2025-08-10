

pub trait AsLib{
    fn get_path<'a>(&'a self)->&'a str;
    fn get_name<'a>(&'a self)->&'a str;
}