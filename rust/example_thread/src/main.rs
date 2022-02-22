fn main() {
    // Create some kind of future that we want our runtime to execute
    let program = my_outer_stream.for_each(|my_outer_value| {
        println!("Got value {:?} from the stream", my_outer_value);
        let my_inner_future = future::ok(1);

        let task = my_inner_future.and_then(|my_inner_value| {
            println!("Got a value {:?} from second future", my_inner_value);
            Ok(())
        });

        tokio::spawn(task);
        Ok(())
    });

    tokio::run(program);
}
