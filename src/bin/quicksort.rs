
use rand::Rng;

fn generate_random_numbers(amount: i32) -> Vec<i32> {
    let mut result = vec![];
    for _ in 0..amount {
        result.push(rand::thread_rng().gen_range(0..100));
    }
    result
}

fn partition(sortable: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
    let pivot = sortable[hi]; 
    let mut i = lo; 
    for j in lo..hi { 
        if sortable[j] <= pivot { 
            let temp  = sortable[i]; 
            sortable[i] = sortable[j]; 
            sortable[j] = temp; 
            i = i+1;
        }
    }
    let temp = sortable[i];
    sortable[i] = sortable[hi];
    sortable[hi] = temp;
    i
}

fn quicksort(sortable:  &mut Vec<i32>, lo: usize, hi: usize ) ->  &Vec<i32> {
    if lo < hi {
        let pi = partition(sortable, lo, hi);
        match pi {
            0 => {
                quicksort(sortable, lo, 0);
                quicksort(sortable, pi+1, hi);
            }
            _ => {
                quicksort(sortable, lo, pi-1);
                quicksort(sortable, pi+1, hi);
            }
        }
    }
    sortable
}

fn main() {
    let mut initial: Vec<i32> = generate_random_numbers(10);
    for (idx, num) in initial.iter().enumerate() {
        print!("{idx}: {num}\n");
    }
    let hi = initial.len() -1;
    let result = quicksort(&mut initial, 0, hi);
    print!("\n\nSorting!!\n\n");
    for (idx, num) in result.iter().enumerate() {
        print!("{idx}: {num}\n");
    }
}