#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        123
    });

    // Do some other work

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}