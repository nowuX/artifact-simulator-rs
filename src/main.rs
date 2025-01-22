use ras::Artifact;

fn main() {
    loop {
        let artifact = Artifact::random();
        print!("\x1B[2J\x1B[1;1H");
        println!("\n{}", artifact);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
