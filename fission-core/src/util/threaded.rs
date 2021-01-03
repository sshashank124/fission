#[inline] pub fn run(f: impl FnOnce() + Send)
{ crossbeam::thread::scope(|s| { s.spawn(move |_| f()); }).unwrap() }
