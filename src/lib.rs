#![feature(test)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        assert!(day1::execute(false) == 1603498);
        assert!(day1::execute(true) == 25574739);
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

    #[test]
    fn test_day4() {
        assert!(day4::execute(false) == 2557);
        assert!(day4::execute(true) == 1854);
    }

    #[test]
    fn test_day5() {
        assert!(day5::execute(false) == 4609);
        assert!(day5::execute(true) == 5723);
    }

    #[test]
    fn test_day6() {
        assert!(day6::execute(false) == 5461);
        assert!(day6::execute(true) == 1836);
    }

    #[test]
    fn test_day7() {
        assert!(day7::execute(false) == 3119088655389);
        assert!(day7::execute(true) == 264184041398847);
    }

    #[test]
    fn test_day8() {
        assert!(day8::execute(false) == 323);
        assert!(day8::execute(true) == 1077);
    }

    #[test]
    fn test_day9() {
        assert!(day9::execute(false) == 6323641412437);
        //assert!(day9::execute(true) == 1077);
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;
    /*
    #[bench]
    fn bench_day1p1(b: &mut Bencher) {
        b.iter(|| day1::execute(false));
    }
    #[bench]
    fn bench_day1p2(b: &mut Bencher) {
        b.iter(|| day1::execute(true));
    }

    #[bench]
    fn bench_day2p2(b: &mut Bencher) {
        b.iter(day2::execute);
    }

    #[bench]
    fn bench_day3p1(b: &mut Bencher) {
        b.iter(|| day3::execute(false));
    }
    #[bench]
    fn bench_day3p2(b: &mut Bencher) {
        b.iter(|| day3::execute(true));
    }

    #[bench]
    fn bench_day4p1(b: &mut Bencher) {
        b.iter(|| day4::execute(false));
    }
    #[bench]
    fn bench_day4p2(b: &mut Bencher) {
        b.iter(|| day4::execute(true));
    }

    #[bench]
    fn bench_day5p1(b: &mut Bencher) {
        b.iter(|| day5::execute(false));
    }

    #[bench]
    fn bench_day5p2(b: &mut Bencher) {
        b.iter(|| day5::execute(true));
    }


    #[bench]
    fn bench_day6p1(b: &mut Bencher) {
        b.iter(|| day6::execute(false));
    }

    #[bench]
    fn bench_day6p2(b: &mut Bencher) {
        b.iter(|| day6::execute(true));
    }



    #[bench]
    fn bench_day7p1(b: &mut Bencher) {
        b.iter(|| day7::execute(false));
    }

    #[bench]
    fn bench_day7p2(b: &mut Bencher) {
        b.iter(|| day7::execute(false));
    }



    #[bench]
    fn bench_day8p1(b: &mut Bencher) {
        b.iter(|| day8::execute(false));
    }

    #[bench]
    fn bench_day8p2(b: &mut Bencher) {
        b.iter(|| day8::execute(false));
    }

    */

    #[bench]
    fn bench_day9p1(b: &mut Bencher) {
        b.iter(|| day9::execute(false));
    }

    #[bench]
    fn bench_day9p2(b: &mut Bencher) {
        b.iter(|| day9::execute(false));
    }
}
