use super::{bin_config::BinConfig, external_config::ExternalConfig, lib_config::LibConfig};



pub struct CmakeConfig{
    project:String,
    version:String,
    cmake_minimum_required:String,
    cpp_standard:String,
    external_library_path:String,
    binaries:Vec<BinConfig>,
    libraries:Vec<LibConfig>,
    dependencies:Vec<ExternalConfig>,


}

impl CmakeConfig {
    fn new(project:String,version:String,cmake_minimum_required:String,cpp_standard:String,external_library_path:String,)->CmakeConfig{
        CmakeConfig { project: project, version: version, cmake_minimum_required: cmake_minimum_required, cpp_standard: cpp_standard, external_library_path: external_library_path, binaries: Vec::new(), libraries: Vec::new(), dependencies: Vec::new() }
    }
}