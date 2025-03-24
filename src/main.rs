use std::sync::{Arc, Mutex, OnceLock};

trait CalculationFunction<T>: Fn() -> T + Send {}

impl<T, F> CalculationFunction<T> for F where F: Fn() -> T + Send {}

struct Calculation<T, F>
where
    F: CalculationFunction<T>,
{
    r: std::sync::OnceLock<T>,
    f: F,
}

impl<T, F> Calculation<T, F>
where
    F: CalculationFunction<T>,
{
    fn new(f: F) -> Self {
        Self {
            r: OnceLock::new(),
            f,
        }
    }

    fn calculate(&self) -> &T {
        self.r.get_or_init(|| (self.f)())
    }
}

fn main() {
    let c = Calculation::new(|| false);
    let cache = Arc::new(Mutex::new(c));

    let spawned_cache = cache.clone();
    std::thread::spawn(move || {
        loop {
            match spawned_cache.lock() {
                Ok(lock) => {
                    let result = lock.calculate();
                    println!("{}", result);
                }
                Err(_) => (),
            }
        }
    })
    .join()
    .unwrap();
}
