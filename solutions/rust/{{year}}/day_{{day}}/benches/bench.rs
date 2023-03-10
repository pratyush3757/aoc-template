#[macro_use]
extern crate criterion;

use aoclib::Solvable;
use criterion::Criterion;

fn part_one_benchmark(c: &mut Criterion) {
	c.bench_function("{{year}} day {{day}} part one", |b| {
		let input = aoclib::reader("{{year}}", "{{day}}", "input.txt").unwrap();
		b.iter(|| aoc_{{year}}_{{day}}::PartOne::solve(&input).unwrap())
	});
}

fn part_two_benchmark(c: &mut Criterion) {
	c.bench_function("{{year}} day {{day}} part two", |b| {
		let input = aoclib::reader("{{year}}", "{{day}}", "input.txt").unwrap();
		b.iter(|| aoc_{{year}}_{{day}}::PartTwo::solve(&input).unwrap())
	});
}

criterion_group!(benches, part_one_benchmark, part_two_benchmark);
criterion_main!(benches);
