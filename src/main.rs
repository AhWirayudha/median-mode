fn main() {
    // vector median
    let vector = vec![1, 2, 4, 5, 6, 7, 1, 4, 6, 8, 3, 4, 7, 3, 5, 4, 1, 6, 8, 9, 5];
    median(&vector);
    println!("{:?}", vector)
}

fn median(v: &Vec<i32>)->i32{
    // sort v
    // len of v
    // if odd then the middle number is median
    // if even then average of 2 middle number (middle pair) is median
    for i in v {
        println!("{:?}", i);
        // logic for median
    };
    5
}
