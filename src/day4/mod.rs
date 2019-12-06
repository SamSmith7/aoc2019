

fn check_adjacent(num: &String<>) -> bool {
    
    let mut prev = '!';
    for digit in num.chars() {
        if digit == prev { return true }
        prev = digit;
    }
    return false
}

fn check_order(num: &String<>) -> bool {

    let mut prev = '!'; // low ascii char so first digit is larger
    for digit in num.chars() {
        if digit < prev { return false }
        prev = digit;
    }
    return true
}

pub fn part1() -> String<> {

    let start = 372304;
    let end = 847060;
    let mut count = 0;

    for num in start..end {
        let num_as_str = num.to_string();
        if check_adjacent(&num_as_str) && check_order(&num_as_str) {
            count += 1;
        }
    }

    count.to_string()
}

fn check_adjacent_pt2(num: &String<>) -> bool {

    let mut group_len = 1;
    let mut prev = '!'; // low ascii char so first digit is larger

    for digit in num.chars() {
        if digit == prev {
            group_len += 1;
        } else if group_len == 2 {
            return true
        } else {
            group_len = 1;
        }
        prev = digit;
    }
    return group_len == 2
}

pub fn part2() -> String<> {

    let start = 372304;
    let end = 847060;
    let mut count = 0;

    for num in start..end {
        let num_as_str = num.to_string();
        if check_adjacent_pt2(&num_as_str) && check_order(&num_as_str) {
            count += 1;
        }
    }

    count.to_string()
}