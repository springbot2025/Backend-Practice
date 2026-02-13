fn main(){
    let mut v:Vec<i32> = vec![];
    let mut sum:i32 = 0;
    for i in 0..11{
        v.push(i);
        sum += i;
    }
    println!("{}",sum);
}