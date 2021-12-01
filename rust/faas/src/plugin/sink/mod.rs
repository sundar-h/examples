pub trait Sink {
    fn initialize();
    fn finalize();
    fn next();
}