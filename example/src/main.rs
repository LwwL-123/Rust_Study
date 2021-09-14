
fn main() {

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
    //
    // let x = students.clone().into_iter().map(|mut x1| {
    //     x1.id += 1;
    //     x1
    // }).collect::<Vec<_>>();
    // println!("{:?}",x);
    //
    // let vec1 = students.into_iter().filter(| x2| {
    //     if x2.id == 2 {
    //         true
    //     }else {
    //         false
    //     }
    // }).collect::<Vec<_>>();
    // println!("{:?}",vec1);




    // 排序
    // let mut vec1 = vec![1, 50, 60, 4, 10, 9, 11, 20, 30];
    // vec1.sort_by(|a, b| b.cmp(&a));
    // println!("{:?}", vec1);


    // match vec1.binary_search(&7) {
    //     Ok(a) => print!("{}",a),
    //     Err(s) => print!("{}",s)
    // }


    let s1 = String::from("qqq");
    let s2 = s1;
    println!("{:?}", s1.as_ptr());
    println!("{:?}", s2.as_ptr());
}

