// use tracing_log::LogTracer;
// use log
use std::sync::atomic::{AtomicUsize, Ordering};

static STATE: AtomicUsize = std::sync::atomic::AtomicUsize::new(2);
const UNINITIALIZED: usize = 0;
const INITIALIZING: usize = 1;
const INITIALIZED: usize = 2;
fn main() {
    // LogTracer::builder()
    //     .ignore_crate("foo") // suppose the `foo` crate is using `tracing`'s log feature
    //     .with_max_level(log::LevelFilter::Info)
    //     .init()?;
    //
    // // will be available for Subscribers as a tracing Event
    // log::info!("an example info log");
    // let a: Vec<String> = Vec::new();
    // let b = a.into_boxed_slice();
    // // println!("{}", b);
    // println!("OKOK")

    let old_state = match STATE.compare_exchange(
        UNINITIALIZED,
        INITIALIZING,
        Ordering::SeqCst,
        Ordering::SeqCst,
    ) {
        Ok(s) | Err(s) => s,
    };
    println!("old_state: {}", old_state);
    // match old_state {
    //     UNINITIALIZED => {
    //         unsafe {
    //             // LOGGER = make_logger();
    //         }
    //         STATE.store(INITIALIZED, Ordering::SeqCst);
    //         Ok(())
    //     }
    //     INITIALIZING => {
    //         while STATE.load(Ordering::SeqCst) == INITIALIZING {
    //             // TODO: replace with `hint::spin_loop` once MSRV is 1.49.0.
    //             #[allow(deprecated)]
    //             std::sync::atomic::spin_loop_hint();
    //         }
    //         Err(SetLoggerError(()))
    //     }
    //     _ => Err(SetLoggerError(())),
    // }
}
