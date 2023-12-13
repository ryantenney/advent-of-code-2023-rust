use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Timer {
    pub duration: Duration,
}

impl Timer {

    pub fn new() -> Timer {
        Timer { ..Default::default() }
    }

    pub fn time(&mut self, f: impl FnOnce()) -> Duration {
        let start = Instant::now();
        f();
        let elapsed = start.elapsed();
        self.duration += elapsed;
        elapsed
    }

    pub fn time_with_return<T>(&mut self, f: impl FnOnce() -> T) -> (T, Duration) {
        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed();
        self.duration += elapsed;
        (result, elapsed)
    }

    pub fn time_with_result<T, E>(&mut self, f: impl FnOnce() -> Result<T, E>) -> Result<(T, Duration), E> {
        let start = Instant::now();
        let result = f()?;
        let elapsed = start.elapsed();
        self.duration += elapsed;
        Ok((result, elapsed))
    }

}
