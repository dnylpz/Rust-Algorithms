// Given an array of integers greater than zero, 
// find if it is possible to split it in two subarrays 
// (without reordering the elements), 
// such that the sum of the two subarrays is the same. 
// Print the two subarrays.
//
// [3,4,5,6, 10,8] -> [3,4,5,6], [10,8]
// [3,3,3,3,5,5,3,3,3,3] -> [3,3,3,3,5] [5,3,3,3,3]
// [3,3,4,1,3,4] - Exception
//


fn find_pivot(list_of_numbers: &Vec<i32>) -> Option<usize> {
    let mut sum = 0;
    for number in list_of_numbers {
        sum += number;
    }
    
    let mut right_sum = 0;
    let mut j = list_of_numbers.len() -1;
    while j != 0  {
        right_sum += list_of_numbers[j];
        sum -= list_of_numbers[j];
        if right_sum == sum {
            return Some(j);
        }
        j -= 1;
    }
    None
}

fn print_arrays(pivot: usize, array: Vec<i32>) {
    print!("Left: [");
    for i in 0..pivot {
        let num = array[i];
        
        print!("{num}, ");
    }
    print!("]\nright[");
    for i in pivot..array.len() {
        let num = array[i];
        print!("{num}, ");
    }
    print!("]\n");
}
fn have_pivot(the_list: Vec<i32>) {
    find_pivot(&the_list).map(|number| {
        print_arrays(number, the_list) 
    });
}

fn main() {
    let can_be_pivot = vec![3,4,5,6,10,8];
    let can_be_pivot2 = vec![3,3,3,3,5,5,3,3,3,3];
    have_pivot(can_be_pivot);
    have_pivot(can_be_pivot2);
}

