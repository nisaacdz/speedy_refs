use num_format::{Locale, ToFormattedString};

pub struct MyBencher(usize);

impl MyBencher {
    pub fn new(size: usize) -> Self {
        Self(size)
    }
    pub fn bench(&self, alias: &str, f: impl Fn()) {
        let r1 = self.measure(&f).to_formatted_string(&Locale::en);
        let r2 = self.measure(&f).to_formatted_string(&Locale::en);
        let r3 = self.measure(&f).to_formatted_string(&Locale::en);

        println!("Results for {}\n[{}ms, {}ms, {}ms]\n", alias, r1, r2, r3);
    }
    fn measure(&self, f: &impl Fn()) -> u128 {
        let clock = std::time::Instant::now();
        self.call(f);
        clock.elapsed().as_millis()
    }
    #[inline]
    fn call(&self, f: impl Fn()) {
        for _ in 0..self.0 {
            f()
        }
    }
}
