// This problem was asked by Quora.
// Given an absolute pathname that may have . or .. as part of it, return the shortest standardized path.
// For example, given "/usr/bin/../bin/./scripts/../", return "/usr/bin/".


fn resolve_path(path: String) -> String {
    let mut result: Vec<&str> = vec![];
    for folder in  path.split('/') {
        if folder.eq("..") {
            result.pop();
        } 
        else if !folder.eq(".") {
            result.push(folder)
        }
    }
    result.join("/")
}


fn main () {
    assert!(resolve_path("/usr/bin/../bin/./scripts/../".to_string()) == "/usr/bin/".to_string());
}
