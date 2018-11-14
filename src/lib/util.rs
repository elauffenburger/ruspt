pub fn log<F>(logFn: F)
where
    F: FnOnce(),
{
    if cfg!(feature = "debug") {
        logFn();
    }
}
