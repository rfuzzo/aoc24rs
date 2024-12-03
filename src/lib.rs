#![feature(test)]

pub mod day1;
pub mod day2;
pub mod day3;

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        assert!(day1::execute() == 1603498);
    }

    #[test]
    fn test_day2() {
        assert!(day2::execute() == 589);
    }

    #[test]
    fn test_day3() {
        assert!(day3::execute(false) == 174960292);
        assert!(day3::execute(true) == 56275602);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_day2(b: &mut Bencher) {
        b.iter(day2::execute);
    }

    #[bench]
    fn bench_day1(b: &mut Bencher) {
        b.iter(day1::execute);
    }

    #[bench]
    fn bench_day3(b: &mut Bencher) {
        b.iter(|| day3::execute(true));
    }
}
