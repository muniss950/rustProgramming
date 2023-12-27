// use std::cmp::PartialOrd;
//
//
// fn main() {
//     let num=vec![34,50,25,100,65];
//     let larges=largest(&num); 
//     println!("The largest number in num is {}",larges);
//     assert_eq!(larges,100 );
// }
// fn largest<T: PartialOrd+Copy>(list: &[T])->T{
//     let mut largest=list[0];
//     for &item in list.iter(){
//         if item>largest{
//             largest=item;
//         }
//     }
//     largest
// }

fn main() {
    let string1=String::from("abcd");
    let result;
    {
    let string2=String::from("xyz");
    result=longest(string1.as_str(),string2.as_str());
    }
    println!("The longest string is {}",result);
}
fn longest< 'a>(x: &'a str,y:&'a str)->&'a str{
    if x.len()>y.len(){
        x
    }
    else {
        y
    }
}
