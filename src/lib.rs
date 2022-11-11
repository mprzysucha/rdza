use std::collections::HashMap;

pub mod exercises {
    use std::collections::HashMap;

    pub fn mean(v: &Vec<i32>) -> i32 {
        if v.len() == 0 {
            return 0;
        }
        let mut sum = 0;
        for i in v {
            sum += i;
        }
        sum / (v.len() as i32)
    }

    pub fn median(v: &Vec<i32>) -> i32 {
        if v.len() == 0 {
            return 0;
        }
        let mut v2 = v.clone();
        v2.sort();
        // println!("v2.len() ")
        if (v2.len() % 2 == 0) {
            (v2[v2.len() / 2 - 1] + v2[v2.len() / 2]) / 2
        } else {
            v2[v2.len() / 2]
        }
    }

    pub fn mode(v: &Vec<i32>) -> i32 {
        if v.len() == 0 {
            return 0;
        }
        let mut counter = HashMap::new();
        for i in v {
            let mut count = counter.entry(i).or_insert(0);
            *count += 1;
        }
        let mut max_elem = v[0];
        let mut max_count = -1;
        println!("counter: {:?}", counter);
        for e in counter {
            if e.1 > max_count {
                max_elem = *e.0;
                max_count = e.1;
            }
        }
        max_elem
    }
}