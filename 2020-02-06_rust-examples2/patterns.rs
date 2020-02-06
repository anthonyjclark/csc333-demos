fn swap(tuple: (i32, i32)) -> (i32, i32) {
    // if tuple.0 == 0 {
    //     (1, 0)
    // } else {
    //     (tuple.1, tuple.0)
    // }
    match tuple {
        (0, _) => (1, 0),
        (first, second) => (second, first),
    }
}

fn main() {
    let (x, y) = (5, 6);
    dbg!(x, y);

    let test1 = swap((10, 20));
    dbg!(test1);

    let (a, b) = swap((10, 20));
    dbg!(a, b);
}
