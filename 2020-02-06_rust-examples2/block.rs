fn main() {
    let y = 3;
    let some_string = "csc333";
    let x = if y > 5 {
        let j = 4;
        let k = 3;
        if j > k {
            "hi"
        } else {
            "5"
        }
    } else {
        match some_string {
            "csc333" => "one",
            _ => "two"
        }
    };

    dbg!(x);
}
