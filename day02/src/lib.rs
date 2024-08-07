#[link(name = "day02", kind = "static")]
extern "C" {
    fn day02_solution_01(input: *const u8, n: u64) -> u64;
    fn day02_solution_02(input: *const u8, n: u64) -> u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        unsafe {
            let result = day02_solution_01(input.as_bytes().as_ptr(), input.len() as u64);
            assert_eq!(result, 8);
        }
    }

    #[test]
    fn example_02() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        unsafe {
            let result = day02_solution_02(input.as_bytes().as_ptr(), input.len() as u64);
            assert_eq!(result, 2286);
        }
    }
}
