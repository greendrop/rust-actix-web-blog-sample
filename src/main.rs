fn main() {
    let result = server::start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
