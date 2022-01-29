use std::fs;

fn main() {
    println!("vv Enter a directory to create the solution in vv");

    let mut dir = String::new();
    std::io::stdin().read_line(&mut dir).expect("Not a valid input!");

    fs::create_dir_all(&dir);

    println!("vv Input a custom name for the solution. (Enter NULL for none) vv");

    let mut univ = String::new();
    std::io::stdin().read_line(&mut univ).expect("Not a valid input!");

    match &univ.trim() {
        &"NULL" => no_name(dir, univ),
        _ => custom_name(dir, univ)
    }
}

fn custom_name(dir: String, univ: String) {
    std::process::Command::new("dotnet")
    .arg("new")
    .arg("sln")
    .arg("-o")
    .arg(&dir.trim())
    .arg("--name")
    .arg(&univ.trim())
    .spawn().expect("couldn't execute");

    std::thread::sleep(std::time::Duration::new(3, 3));

    println!("vv Would you like to create a project for this solution? [y/n] vv");

    let mut yorn = String::new();
    std::io::stdin().read_line(&mut yorn).expect("Not a valid input!");

    if &yorn.trim() == &String::from("y") {
        
        println!("vv What type of project do you want it to be? vv");
        
        let mut projtype = String::new();
        std::io::stdin().read_line(&mut projtype).expect("Not a valid input!");

        std::process::Command::new("dotnet")
            .arg("new")
            .arg(&projtype.trim())
            .arg("-o")
            .arg(&dir.trim())
            .arg("--name")
            .arg(&univ.trim())
            .spawn().expect("couldn't execute");

        std::thread::sleep(std::time::Duration::new(3, 3));

        std::process::Command::new("dotnet")
            .arg("sln")
            .arg(&dir.trim())
            .arg("add")
            .arg(&dir.trim())
            .spawn().expect("couldn't execute");
    }
}

fn no_name(dir: String, univ: String) {
    std::process::Command::new("dotnet")
        .arg("new")
        .arg("sln")
        .arg("-o")
        .arg(&dir.trim())
        .spawn().expect("couldn't execute");

    std::thread::sleep(std::time::Duration::new(3, 3));

    println!("vv Would you like to create a project for this solution? [y/n] vv");

    let mut yorn = String::new();
    std::io::stdin().read_line(&mut yorn).expect("Not a valid input!");

    if &yorn.trim() == &String::from("y") {
        
        println!("vv What type of project do you want it to be? vv");
        
        let mut projtype = String::new();
        std::io::stdin().read_line(&mut projtype).expect("Not a valid input!");

        std::process::Command::new("dotnet")
            .arg("new")
            .arg(&projtype.trim())
            .arg("-o")
            .arg(&dir.trim())
            .spawn().expect("couldn't execute");

        std::thread::sleep(std::time::Duration::new(3, 3));

        std::process::Command::new("dotnet")
            .arg("sln")
            .arg(&dir.trim())
            .arg("add")
            .arg(&dir.trim())
            .spawn().expect("couldn't execute");
    }
}
