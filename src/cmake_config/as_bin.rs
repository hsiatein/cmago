
pub trait AsBin{
    fn get_path<'a>(&'a self)->&'a String;
}