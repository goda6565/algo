fn linear_search(target_list: &Vec<usize>, target: &usize) -> bool {
    for i in target_list {
        if i == target {
            return true;
        }
    }
    false
}

fn binary_search(target_list: &Vec<usize>, target: &usize) -> bool {
    let mut low = 0;
    let mut high = target_list.len() - 1;
    while low <= high {
        let mid = (low + high) / 2;
        let guess = target_list[mid];
        if guess == *target {
            return true;
        }
        if guess > *target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    false
}

fn interpolation_search(target_list: &Vec<usize>, target: &usize) -> bool {
    let mut low = 0;
    let mut high = target_list.len() - 1;
    while low <= high {
        let mid = low
            + ((high - low) / (target_list[high] - target_list[low]))
                * (*target - target_list[low]) as usize;
        let guess = target_list[mid];
        if guess == *target {
            return true;
        }
        if guess > *target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    false
}

fn main() {
    let target_list = &vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = &5;
    let result = linear_search(target_list, target);
    println!("{}", result);
    let result = binary_search(target_list, target);
    println!("{}", result);
    let result = interpolation_search(target_list, target);
    println!("{}", result);
}
