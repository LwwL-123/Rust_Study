use std::collections::HashMap;
fn main() {

    // ///结构体
    // #[derive(Debug,Clone)]
    // struct student{
    //     name: String,
    //     id: u64,
    // }
    //
    // let mut students:Vec<student> = vec![];
    // for i in 1..5{
    //     students.push(student {
    //         name: "lzw".to_string(),
    //         id: i,
    //     });
    // }
    //
    // ///map
    // let x = students.clone().into_iter().map(|mut x1| {
    //     x1.id += 1;
    //     x1
    // }).collect::<Vec<_>>();
    // println!("{:?}",x);
    //
    // ///filter
    // let vec1 = students.into_iter().filter(| x2| {
    //     if x2.id == 2 {
    //         true
    //     }else {
    //         false
    //     }
    // }).collect::<Vec<_>>();
    // println!("{:?}",vec1);
    //
    // ///sort
    // let mut vec1 = vec![1, 50, 60, 4, 10, 9, 11, 20, 30];
    // vec1.sort_by(|a, b| b.cmp(&a));
    // println!("{:?}", vec1);
    //
    // ///二分查找
    // match vec1.binary_search(&7) {
    //     Ok(a) => print!("{}",a),
    //     Err(s) => print!("{}",s)
    // }


    ///hashmap
    let mut map = HashMap::new();
    map.insert(String::from("lzw"),10);
    map.insert(String::from("lzww"),100);

    let value = map.get("lzw");
    println!("{:?}", value.unwrap());


}

#[test]
fn test1(){
    let a = 1;
    assert_eq!(a,1)
}
