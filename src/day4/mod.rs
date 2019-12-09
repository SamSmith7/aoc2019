
fn check_adjacent(num: &Counter) -> bool {
    let mut prev = 0;
    for digit in num.digits.clone() {
        // print!("{}", digit);
        if digit == prev { return true }
        prev = digit;
    }
    return false
}

fn check_order(num: &Counter) -> bool {
    let mut prev = 0;
    for digit in num.digits.clone() {
        if digit < prev { return false }
        prev = digit;
    }
    return true
}

struct Counter {
    digits: Vec<i32>
}

impl Counter {
    fn new(int: i32) -> Counter {
        let mut digits: Vec<i32> = vec![];
        let mut num = int;
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.reverse();
        Counter { digits }
    }

    fn _increment(&mut self, idx: usize) {
        let digit = self.digits[idx];
        if digit + 1 > 9 {
            self.digits[idx] = 0;
            self._increment(idx - 1);
        } else {
            self.digits[idx] = digit + 1;
        }
    }

    fn increment(&mut self) {
        self._increment(5);
    }
}


pub fn part1() -> String<> {

    let start = 372304;
    let end = 847060;
    let mut count = 0;
    let mut counter = Counter::new(start);

    for _num in start..end {
        if check_adjacent(&counter) && check_order(&counter) {
            count += 1;
        }
        counter.increment();
    }

    count.to_string()
}

fn check_adjacent_pt2(num: &Counter) -> bool {

    let mut group_len = 1;
    let mut prev = 0;

    for digit in num.digits.clone() {
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
    let mut counter = Counter::new(start);

    for _num in start..end {
        if check_adjacent_pt2(&counter) && check_order(&counter) {
            count += 1;
        }
        counter.increment();
    }

    count.to_string()
}