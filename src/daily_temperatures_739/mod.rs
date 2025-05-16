
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    // We can use monotonic increasing stack to solve this problem

    let mut result = vec![0; temperatures.len()];
    // We create a fixed array of integers that represent the number of days you have to wait after that day to get a warmer temperature
    // We initially set the number of days to 0 and we will calculate the days gap with the monotonic increasing stack
    let mut stack: Vec<usize> = Vec::new();

    // We loop through the array of temperatures
    for (t_index, t_val) in temperatures.iter().enumerate() {
        // For every day that is warmer to the previous day, we get the difference in days using the array indices
        // For example
        // [73, 74] => we refer to its array indices to represent the day number, [day 0, day 1]
        // For the day with temperature of 73 to wait for the next warmer day, which is day 1 (temperature of 74)
        // We simply calculate the difference (day 1 - day 0 => 1)
        // Another example
        // [75, 71, 69, 72, 76] => convert to day numbers => [0, 1, 2, 3, 4]
        // Answer is 4 days for the day with temperature of 75 to wait until the next warmer day
        while stack.len() > 0 && &temperatures[stack.last().unwrap().clone()] < t_val {
            // With the monotonic increasing stack, we can "backtrack" and calculate the difference in days
            // This is how the stack will look while moving through the temperatures
            // [75] => pick up day 0 - temp 75, doesn't do any popping since its the only element
            // [75, 71] => pick up day 1 - temp 71, doesn't do any popping since the element (71) is not bigger than the last element (75) in the stack
            // [75, 71, 69] => pick up day 2 - temp 69, doesn't do any popping since the element (69) is not bigger than the last element (71) in the stack
            // [75, 72] => when it reaches 72, it keeps popping the last element until the element is larger than 72
            // When it keeps popping the last element until a larger element comes, we are also calculating the difference in days
            // result[day 2 - temp 69] = day 3 - day 2 = 1
            // result[day 1 - temp 71] = day 3 - day 1 = 2
            // [76] => when it reaches 76, it keeps popping the last element until the element is larger than 76
            // And it will start calculating the difference in days
            // result[day 3 - temp 72] = day 4 - day 3 = 1
            // result[day 0 - temp 75] = day 4 - day 0 = 4
            // Your end result will then be [4, 2, 1, 1, 0] since no element is larger than 76 
            let r_index = stack.pop().unwrap();
            result[r_index] = (t_index - r_index) as i32;
        }
        stack.push(t_index);
    }
    return result;
}


mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let result = daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
        assert_eq!(result, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }

    #[test]
    fn test_case_2() {
        let result = daily_temperatures(vec![30, 40, 50, 60]);
        assert_eq!(result, vec![1, 1, 1, 0]);
    }

    #[test]
    fn test_case_3() {
        let result = daily_temperatures(vec![30, 60, 90]);
        assert_eq!(result, vec![1, 1, 0]);
    }
}