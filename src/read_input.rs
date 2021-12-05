use std::fs;

pub fn read_input(path: &str) -> String {
    let current_exec_path = std::env::current_exe().unwrap();
    let mut folder = Vec::from_iter(current_exec_path.to_str().unwrap().split('/'));

    folder.pop();

    let path = folder.join("/") + "/" + path;

    fs::read_to_string(path).unwrap()
}
