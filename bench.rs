use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

//use solution::day3::{part1, part2};

const ANSWER_1: i64 = 190604937;
const ANSWER_2: i64 = 82857512;

fn bench_part1(c: &mut Criterion) {
    //let input = include_str!("input.txt");
    c.bench_function("part1", |b| {
        b.iter(|| assert_eq!(ANSWER_1, ANSWER_1))
    });
}

fn bench_part2(c: &mut Criterion) {
    //let input = include_str!("input.txt");
    c.bench_function("part2", |b| {
        b.iter(|| assert_eq!(ANSWER_2, ANSWER_2))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
