pub fn square_of_sum(num: i32) -> i32 {
    let mut sum = 0;
    for x in 0..num {
        sum += x + 1;
    }
    return sum.pow(2);
}

pub fn sum_of_squares(num: i32) -> i32 {
    let mut sum = 0;
    for x in 0..num {
        sum += (x + 1).pow(2);
    }
    return sum;
}

pub fn difference(num: i32) -> i32 {
    return square_of_sum(num) - sum_of_squares(num);
}
