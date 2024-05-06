use ags::generate_artifact;

fn main() {
    loop {
        let artifact = generate_artifact(false);
        print!("\x1B[2J\x1B[1;1H");
        println!("\n{}", artifact);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
