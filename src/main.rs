mod file_reader;

fn main() {
    println!("Start Auto Commit ...");

    file_reader::file_reader::run();

    println!("Done Auto Commit ...");


}
