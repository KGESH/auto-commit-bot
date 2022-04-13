/* Todo: Extract function */
pub mod file_reader {
    use std::{fs, io, process};
    use std::fs::File;
    use std::io::{Read, Write};
    use std::process::{Child, ExitStatus, Output};
    use std::thread::sleep_ms;

    fn check_txt_file(file_name: &str) -> String {
        let file = fs::OpenOptions::new()
            .read(true)
            .open(file_name);

        match file {
            Ok(_) => {
                println!("Success found file");
                let mut url = String::new();
                file.unwrap().read_to_string(&mut url);
                url
            }
            Err(_) => {
                println!("Can not found {}", &file_name);
                println!("Create new file...");
                print!("Please input your github repository: ");
                io::stdout().flush().expect("Can not flush");

                let mut url = String::new();
                if let Err(err) = io::stdin().read_line(&mut url) {
                    println!("Can not read url - {}", err);
                }

                let mut file = fs::File::create(&file_name).expect("Can not create file");
                write!(file, "{}", &url).expect("Can not write url");
                url
            }
        }
    }

    fn input_remote_repository_url(file_name: &str, url: &str) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .open(file_name).unwrap();

        write!(file, "{}", &url);
    }

    fn append_string_to_file(file_name: &str, content: &str) {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_name)
            .unwrap();

        writeln!(file, "{}", &content);
    }

    fn clone_repository(url: &str) -> std::io::Result<ExitStatus> {
        let mut clone_command = "git clone ".to_owned() + url;
        let mut clone_child = process::Command::new("sh")
            .arg("-c")
            .arg(clone_command)
            .spawn().unwrap();

        clone_child.wait()
    }


    pub fn run() {
        let file_name = "project_repo_url.txt";
        let url = check_txt_file(&file_name);

        match clone_repository(&url) {
            Ok(_) => {
                println!("Success Clone Repo");
                append_string_to_file("rust-study/auto-commit.log", &chrono::Local::now().to_string());

                /* Todo: Extract function */
                let mut add_result = process::Command::new("sh")
                    .arg("-c")
                    .current_dir("rust-study")
                    .arg("git add .")
                    .spawn().unwrap().wait();

                let mut commit_result = process::Command::new("sh")
                    .arg("-c")
                    .current_dir("rust-study")
                    .arg("git commit -m 'auto-commit'")
                    .spawn().unwrap().wait();

                let mut push_result = process::Command::new("sh")
                    .arg("-c")
                    .current_dir("rust-study")
                    .arg("git push origin main")
                    .spawn().unwrap().wait();

                let mut remove_result = process::Command::new("sh")
                    .arg("-c")
                    .arg("rm -rf rust-study")
                    .spawn().unwrap().wait();
            }

            Err(_) => {
                panic!("Error Clone Repo!");
            }
        }
    }
}