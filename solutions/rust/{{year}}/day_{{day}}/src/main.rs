use aoclib::Solvable;
use aoc_{{year}}_{{day}}::{PartOne, PartTwo};

pub fn main() -> aoclib::Result<()> {
    let input = aoclib::reader({{year}}, {{day}}, "input.txt")?;
    let result_part_one = PartOne::solve(&input)?; // 0, ~0μs
    let result_part_two = PartTwo::solve(&input)?; // 0, ~0μs

    println!(
        "Results for {{year}} Day {{day}}:\n\tPart One: {:?}\n\tPart Two: {:?}",
        result_part_one, result_part_two
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_{{year}}_{{day}};
    use aoclib::Solvable;

    fn reader() -> String {
        aoclib::reader({{year}}, {{day}}, "input.txt").unwrap()
    }

    #[test]
    #[ignore]
    fn aoc_{{year}}_{{day}}_part_one() {
        assert_eq!(aoc_{{year}}_{{day}}::PartOne::solve(&reader()).unwrap(), 0);
    }

    #[test]
    #[ignore]
    fn aoc_{{year}}_{{day}}_part_two() {
        assert_eq!(aoc_{{year}}_{{day}}::PartTwo::solve(&reader()).unwrap(), 0);
    }
}
