#![feature(test)]

pub mod day1;
pub mod day2;

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_day1() {
        day1::execute();
    }

    #[test]
    fn test_day2() {
        assert!(day2::execute() == 589);
    }

    #[bench]
    fn bench_day2(b: &mut Bencher) {
        b.iter(day2::execute);
    }

    #[bench]
    fn bench_day1(b: &mut Bencher) {
        b.iter(day1::execute);
    }
}
