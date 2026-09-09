fn main() {
    let mut vec: Vec<i32> = Vec::with_capacity(100);
    vec.push(10);
    println!("{:?}", vec);
    vec.pop();
    println!("{:?}", vec);

    let vec1: Vec<char> = vec!['a', 'b', 'c'];

    println!("{:?}", vec1[0]);

    #[derive(Debug)]
    #[allow(dead_code)]
    enum DifferentTypes {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let values: Vec<DifferentTypes> = vec![
        DifferentTypes::Int(12),
        DifferentTypes::Float(12.21),
        DifferentTypes::Text(String::from("Hello")),
    ];

    println!("{:?}", values);
}
