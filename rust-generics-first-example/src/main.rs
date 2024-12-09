// pub fn sum_of_elements(nums: &[i32]) -> i32 {
//     let mut sum = nums[0];

//     for index in 1..nums.len() {
//         sum += nums[index];
//     }

//     sum
// }

pub fn sum_of_elements_generic<T: std::ops::AddAssign + Copy>(nums: &[T]) -> T {
    let mut sum = nums[0];
    
    for index in 1..nums.len(){
        sum += nums[index]
    }
    
    sum
}

fn main () {
    let my_nums = [1, 2, 3, 4, 5];
    let your_nums = [6, 7, 8, 9, 10];
    // let my_sum = sum_of_elements(&my_nums);
    // let your_sum = sum_of_elements(&your_nums);
    let my_sum = sum_of_elements_generic::<i32>(&my_nums);
    let your_sum = sum_of_elements_generic::<i32>(&your_nums);
    

    let my_i64_nums: [i64; 5] = [11, 12, 13, 14, 15];
    let my_i64_sum = sum_of_elements_generic::<i64>(&my_i64_nums);

    println!("sums = {my_sum}, {your_sum}, {my_i64_sum} ");

    let v: Vec<i32> = vec![1, 2, 3, 4];
}