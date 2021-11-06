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

    let res = 5;
    let mut vec = vec![5];

    let r = vec.clone().into_iter().filter(|x|{
        if x == &res {
            false
        }else {
            true
        }
    }).collect::<Vec<i32>>();


    for i in r {
        println!("{:?}",i);
    }




    // ///hashmap
    // let mut map = HashMap::new();
    // map.insert(String::from("lzw"),10);
    // map.insert(String::from("lzww"),100);
    // map.entry(String::from("lzww")).or_insert(1000);
    // map.entry(String::from("lzwww")).or_insert(1000);
    // let value = map.get("lzw");
    //
    // println!("{:?}", map);
    // println!("{:?}", value.unwrap());


}


