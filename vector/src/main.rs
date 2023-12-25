fn main() {
    let mut v:Vec<i32>=Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let a=vec![1,22,33];
    let mut i=0;
    while i<3{
        println!("{} {}",&v[i],&a[i]);
        i+=1;
    }

}
