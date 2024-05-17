fn main() {
    // vector median
    let mut vector = vec![1, 2, 4, 5, 6, 7, 1, 4, 6, 8, 3, 4, 7, 3, 5, 4, 1, 6, 8, 9, 5];
    let mut vector_two = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut vector_odd = vec![1, 2, 3];
    let mut vector_even = vec![1, 2, 3, 4];
    median(&mut vector_two);
    println!("{:?}", vector);
    mode(&mut vector);
}

fn median(v: &mut Vec<i32>) -> i32 {
    // sort v
    println!("before : {:?}", v);
    let sorted = v;
    sorted.sort();
    println!("sorted : {:?}", sorted);
    // mod % 2 len of v
    match sorted.len() % 2 {
        0 => {
            println!("even {:?}", sorted.len());
            let left = sorted.len() / 2;
            let right = sorted.len() / 2 + 1;
            println!("even median is {:?} {:?}", left, right);
            println!("middle number is {:?}", (sorted[left] + sorted[right]) / 2);
        },
        1 => {
            println!("odd {:?}", sorted.len());
            println!("odd median is {:?}", sorted.len() / 2);
            println!("middle number is {:?}", sorted[sorted.len() / 2]);
        },
        _ => {
            println!("error");
        }
    }
    // if odd then the middle number is median
    // if even then average of 2 middle number (middle pair) is median => middle pair / 2
    for i in sorted {
        println!("{:?}", i);
        // logic for median
    };
    5
}

fn mode(v: &mut Vec<i32>) -> Vec<i32> {
    //snip
    v.to_vec()
}
