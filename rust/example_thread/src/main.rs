fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(hello());
}

async fn hello() {
    println!("begin task");
    // let _ = std::thread::Builder::new().spawn(run());
    // let _ = tokio::spawn(move || {
    //     run();
    // });
}

async fn run() {
    println!("begin run");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    println!("Hello World!")
}
