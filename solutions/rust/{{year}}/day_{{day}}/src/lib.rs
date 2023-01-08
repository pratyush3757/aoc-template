pub struct PartOne;
pub struct PartTwo;

impl aoclib::Solvable<&str, i32> for PartOne {
    fn solve(input: &str) -> aoclib::Result<i32> {
        Ok(0)
    }
}

impl aoclib::Solvable<&str, i32> for PartTwo {
    fn solve(input: &str) -> aoclib::Result<i32> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::{PartOne, PartTwo};
    use aoclib::Solvable;
    use test_case::test_case;

    #[test_case("part_two_input", 0; "sample_1")]
    fn aoc_{{year}}_{{day}}_part_one_samples(input: &str, result: i32) {
        assert_eq!(PartOne::solve(input).unwrap(), result);
    }

    #[test_case("part_two_input", 1; "sample_1")]
    fn aoc_{{year}}_{{day}}_part_two_samples(input: &str, result: i32) {
        assert_eq!(PartTwo::solve(input).unwrap(), result);
    }
}
