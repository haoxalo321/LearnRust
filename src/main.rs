enum Human {
    Eat,
    GoWC,
    PlayGame,
}

struct S {
    aligned: u8,
    unaligned: u32,
}

struct SinhVien {
    msv: i32,
    ten: String,
    diem: i32,
}


use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let x: u64 = "42".parse().expect("Not number");
    print!("{}", x);

    println!("Hello, world!");

    if x == 42 {
        println!("Hello, world!");
    }

    let mut n: u64 = 0;

    loop {
        n += 1;
        println!("in loop",);
        if n >= 10 {
            break;
        }
    }
    println!("{:?}", n);

    println!("While loop");
    while n > 0 {
        n -= 1;
        println!("{:?}", n);
    }
    println!("for loop");
    for i in 1..11 {
        println!("{:?}", i);
    }

    let animals = vec!["dog", "cat", "pig"];

    for (i, a) in animals.iter().enumerate() {
        println!("the number is {:?} the animal name is {:?}", i, a);
    }

    let s = String::from("some|not|necessarily|long|name");
    let last = s.split('|').last().unwrap();

    assert_eq!(last, "name");

    let hao: Human = Human::Eat;

    match hao {
        Human::Eat => {
            println!("Hao dang an ban ei");
        }
        Human::GoWC => {
            println!("Hao dang ia ban ei");
        }
        Human::PlayGame => {
            println!("Hao dang choi dien tu ban ei");
        }
    }

    //const hang so

    let tups = (20, "hao", (2, 3, 4));

    println!("{:?}", (tups.2).1);

    let tups2 = (1, 2, 3);
    let (a, b, c) = tups2;

    println!("{:?}", a);

    let a = [1, 2, 3, 4, 5];

    println!("Nhap thu tu");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("");

    let index: usize = index.trim().parse().expect("");

    let element = a[index - 1];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

    let mut b = [0; 10];

    for i in 0..b.len() {
        b[i] = i + 1;
        println!("{:?}", b[i]);
    }

    for i in b {
        println!("in array");
        println!("{:?}", i);
    }

    let h1 = 3;
    let h2 = 1 + 2;
    println!("{:?}", assert_eq!(h1, h2));

    assert_eq!(h1, h2, "we are testing addition with {} and {}", h1, h2);

    // Request number
    println!("Enter number? ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let bint: usize = input
        .trim()
        .parse()
        .ok()
        .expect("Program only processes numbers, Enter number");
    println!("Square {}", bint * bint); // use number

    let u11: usize = bint;

    let mut c = vec![0; u11];

    for i in 0..c.len() {
        println!("{:?}", c[i]);
    }

    print_number_to(20);

    println!("{:?}", is_max(11));

    let mut x = 10;

    let mut domP: &i32 = &mut x;

    let mut dom: i32 = *domP;

    // *dom = *dom + 1;

    println!("{}", dom);

    // Iterators can be collected into vectors
    let mut collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert new element at the end of the vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Error! Immutable vectors can't grow
    // collected_iterator.push(0);
    // FIXME ^ Comment out this line

    // The `len` method yields the number of elements currently stored in a vector
    println!("Vector length: {}", xs.len());

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());

    // Out of bounds indexing yields a panic
    // println!("Fourth element: {}", xs[3]);
    // FIXME ^ Comment out this line

    // `Vector`s can be easily iterated over
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // A `Vector` can also be iterated over while the iteration
    // count is enumerated in a separate variable (`i`)
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // Thanks to `iter_mut`, mutable `Vector`s can also be iterated
    // over in a way that allows modifying each value
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);

    println!("{:?}", chen_x(&mut collected_iterator, 4));

    println!("Updated vector: {:?}", collected_iterator);

    xoa_x(&mut collected_iterator);

    println!("Updated vector: {:?}", collected_iterator);

    // Increment via closures and functions.
    fn function(i: i32) -> i32 {
        i + 1
    }

    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());

    let s45 = S {
        unaligned: 1,
        aligned: 0,
    };

    let p11 = std::ptr::addr_of!(s45.aligned);
    let p23: &u32 = &s45.unaligned;

    println!("{:?}", p11);

    let mut sinhvien_list: Vec<SinhVien> = Vec::new();

    read_file_simple(&mut sinhvien_list);

    for i in sinhvien_list.iter() {
        println!("Msv: {}, TenSV: {}, DiemSV: {}", i.msv, i.ten, i.diem);
    }

    read_file_self();

    // unsafe {
    //     let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
    //     let (prefix, shorts, suffix) = bytes.align_to::<u16>();
    //     for i in prefix.iter(){
    //         println!("{}", i);
    //     }
    //     for i in shorts.iter(){
    //         println!("{}", i);
    //     }
    //     for i in suffix.iter(){
    //         println!("{}", i);
    //     }
    // }

    let mut sinhvien_last = sinhvien_list.last();

    let mut sinhvien_score_max_list: Vec<&mut SinhVien> = Vec::new();

    sinhvien_score_max_list = select_list_sinhvien_diem_max(&mut sinhvien_list);

    sinhvien_score_max_list.sort_by(|a , b | a.ten.cmp(&b.ten));
    
    for i in sinhvien_score_max_list.iter() {
        println!("Msv: {}, TenSV: {}, DiemSV: {}", i.msv, i.ten, i.diem);
    }

    let x19 = &mut 19; // con tro 
    *x19+=1;
    println!("{}", &x19);








}



fn select_list_sinhvien_diem_max(sinhvien_list: &mut Vec<SinhVien>) -> Vec<&mut SinhVien> {
    sinhvien_list.sort_by(|a: &SinhVien, b: &SinhVien| a.diem.cmp(&b.diem));
    let mut sinhvien_score_max_list: Vec<&mut SinhVien> = Vec::new();
    let max_score = sinhvien_list.last().unwrap().diem;
    
    for i in sinhvien_list.iter_mut() {
        if i.diem >= max_score {
            sinhvien_score_max_list.push(i);
        }
    }

    return sinhvien_score_max_list;
}

fn print_number_to(num: u32) {
    for n in 10..num {
        println!("{:?}", n);
    }
}

fn xoa_x(arr: &mut Vec<i32>) {
    println!("Delete number");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse().ok().expect("Not number");
    let mut count: usize = 0;
    let len: usize = arr.len();

    for i in 0..len {
        for j in i + 1..len {
            if arr[j] == number && j != len - 1 {
                let mut n: usize = j;
                while n < len - 1 {
                    arr[n] = arr[n + 1];
                    n += 1;
                }
                count += 1;
            }
        }
        if arr[i] == number && i == len - 1 {
            count += 1;
        }
    }

    for _i in 0..count {
        arr.pop();
    }
}

fn create_file() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn read_file_simple(sinhvien_list: &mut Vec<SinhVien>) {
    let f = File::open("foo.txt").expect("cant open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let per_line = line.as_ref().unwrap();
        let mut per_line = per_line.split("|");

        let sinh_vien = SinhVien {
            msv: per_line.next().unwrap().parse::<i32>().expect("Not number"),
            ten: per_line.next().unwrap().to_string(),
            diem: per_line.next().unwrap().parse::<i32>().expect("Not number"),
        };

        sinhvien_list.push(sinh_vien);
    }

    // let result = (0..10)
    //     .flat_map(|_| {
    //         let vec: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
    //         vec.into_iter()
    //     })
    //     .collect::<Vec<_>>();

    // println!("{:?}", result);
}

fn read_file_self() -> std::io::Result<()> {
    let text = "foo|bar|baz";
    let mut lines = text.split("|");

    println!("{}", lines.next().unwrap());
    println!("{}", lines.next().unwrap());
    println!("{}", lines.next().unwrap());

    Ok(())
}

fn chen_x(arr: &mut Vec<i32>, x: usize) {
    println!("Enter number? ");
    let mut input = String::new();

    println!("Enter posion? ");
    let mut input2 = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    let bint: i32 = input
        .trim()
        .parse()
        .ok()
        .expect("Program only processes numbers, Enter number");

    let mut len: usize = arr.len();
    while len >= x {
        if len == arr.len() {
            arr.push(arr[len - 1]);
            len += 1;
        }
        if len == x {
            arr[len] = bint;
        } else {
            arr[len - 1] = arr[len - 2];
        }
        len -= 1;
    }
}

fn is_max(num: u32) -> bool {
    return num > 10;
}
