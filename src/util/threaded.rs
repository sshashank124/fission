use crossbeam::thread;

#[inline(always)] pub fn run(f: impl FnOnce() + Send)
{ thread::scope(|s| { s.spawn(move |_| f()); }).unwrap() }
