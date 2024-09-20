fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);

    println!("The largest number is {}", result);
}

fn largest(nums: &[i32]) -> i32 {
    let mut largest = nums[0];

    for &number in nums.iter().skip(1) {
        if number > largest {
            largest = number;
        }
    }

    largest
}
