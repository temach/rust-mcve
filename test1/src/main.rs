use std::path::Path;
use std::string::String;
use std::vec::Vec;

fn templates(pat: &&Path) -> bool {
    let fpath = pat.to_string_lossy();
    fpath.contains("template")
}

fn main() {
    let p1: &Path = Path::new("./");
    let p2: &Path = Path::new("./hello.txt");
    let p3: &Path = Path::new("./template.html");
    let p4: &Path = Path::new("./other.png");

    let v: Vec<&Path> = vec![&p1, &p2, &p3, &p4];

    println!("before: {:?}", v);
    let after: Vec<&&Path> = v
        .iter()
        .filter(|&&fpath| {
            let spath = fpath.to_string_lossy();
            spath.contains("template")
        })
        .collect();
    println!("after: {:?}", after);
}
