

pub struct TestsConfig {
    pub path: String,
}

impl TestsConfig {
    pub fn new()->Self{
        TestsConfig { path: "".to_string() }
    }

    pub fn set_path(&mut self, path:&str){
        self.path = path.to_string();
    }
}