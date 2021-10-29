use std::io::{self, Write};

fn num(k: &mut i64) {
    let mut buf = String::new();
    println!("Enter the number of disks");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut buf)
        .expect("failed  to read input!");

    *k = buf
        .trim()
        .parse::<i64>()
        .expect("Failed to parse input as an integer!");
}

fn toh(num_disks: i64, src: &str, des: &str, buf: &str, num_moves: &mut i64) {
    match num_disks {
        0 => return,
        n => {
            toh(n - 1, src, buf, des, num_moves);
            move_it(num_moves, src, des);
            toh(n - 1, buf, des, src, num_moves);
        }
    }
}

fn move_it(num_moves: &mut i64, src: &str, des: &str) {
    *num_moves = *num_moves + 1; // incrasing number of moves executed so far
    println!("{:>10}: {} -> {}", num_moves, src, des);
}

fn main() {
    let mut num_disks = 0;
    let mut num_moves = 0;
    num(&mut num_disks);
    if num_disks >= 0 {
        println!("No. of Disks = {}", num_disks);
        toh(num_disks, "a", "b", "c", &mut num_moves);
        println!("Number of moves = {}", &num_moves);
    } else {
        println!("Invalid number ({}) of disks", &num_disks);
    }
}
