#![feature(test)]

pub mod day1;
pub mod day2;

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
}
