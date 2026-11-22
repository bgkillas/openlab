let i = 4;
let n = 2;
loop {
    mut n = n * n;
    mut i = i - 1;
    if i == 0 {
        return n;
    }
}