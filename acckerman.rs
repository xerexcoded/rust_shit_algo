fn ackermann(m: usize , n: usize) -> usize {
    match(m,n){
        (0,n) => n+1,
        (m,0) => ackermann(m-1, 1),
        (m,n) => ackermann(m-1,ackermann(m,n-1))
    }
}
fn main() {
    println!("____+____Ackermann function calculation:____+____");
    for m in 0..5 {
        for n in 0..(16-m) {
            println!("ackermann({}, {})",m,n,ackermann(m, n));
        }
    }
}
