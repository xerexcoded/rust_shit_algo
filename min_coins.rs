fn main() {
 let mut much = [0;22]; // array of zeros of size x , where [0,x]
 let coins = [1,5,10,25,50]; // denominations [paisa]
 for x in 0..much.len()
 {
    let amount=x+1;
    if amount<coins[0]
        {
        continue
        }
    if coins.contains(&amount)
        {
        much[x]=1;
        continue
        }
    let mut min=1000000; // just to set the equivalent to -infinity
    for j in 0..coins.len(){
        if coins[j]>amount
        {
        break
        }
        let potent= much[x-coins[j]]+1;
        if potent < min 
        {
        min=potent
        }
    }
    much[x]=min
  }
  println!("Optimal number of coins for {}: {}", much.len(),much[much.len()-1])
}
