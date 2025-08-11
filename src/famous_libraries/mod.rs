
pub fn find_external(name:&str) ->Option<&str>{
    if let Some(item) = FINDER_MAP.iter().find(|item| item.0==name){
        return Some(&item.1);
    }
    None
}

const FINDER_MAP: [(&str, &str); 5] = [
    ("gtest", "https://github.com/google/googletest.git"),
    ("highs", "https://github.com/ERGO-Code/HiGHS.git"),
    ("argparse", "https://github.com/p-ranav/argparse.git"),
    ("polyscope","https://github.com/nmwsharp/polyscope.git"),
    ("xlnt","https://github.com/tfussell/xlnt.git"),
];