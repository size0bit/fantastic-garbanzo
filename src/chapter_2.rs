use std::collections::{HashMap, VecDeque};

//-----------------------------------------------------
pub fn code2_1() {
    let x = 3;
    //x = 5;
    //println!("x: {}", x);
}

//-----------------------------------------------------
pub fn code2_2() {
    let mut x = 3;
    x = 5;
    println!("x: {}", x);
}

//-----------------------------------------------------
pub fn code2_3() {
    let x = 3;
    let x = x + 2;
    let x = x * 2;
    println!("x: {}", x);
    let x = "Hello, Rust!";
    println!("x: {}", x);
}

//-----------------------------------------------------
pub fn code2_4() {
    print!("(1..5): ");
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();
    print!("(1..=5).rev: ");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();
    let sum: i32 = (1..=5).sum();
    println!("1 + 2 + 3 + 4 + 5 = {}", sum);
}

//-----------------------------------------------------
pub fn code2_5() {
    let tup1: (i8, f32, bool) = (-10, 7.7, false);
    let tup2 = (7.7, (false, 10));
    let tup3 = (100, );
    println!("{}, {}", tup1.0, (tup2.1).1);
    println!("{}", tup3.0);
    let (x, y, z) = tup1;
    println!("x: {}, y: {}, z: {}", x, y, z);
}

//-----------------------------------------------------
pub fn code2_6() {
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5];
    let arr3: [i32; 5] = [1; 5];
    let arr4 = [1; 5];
    println!("{:?}", arr1);
    println!("{:?}", arr2);
    println!("{:?}", arr3);
    println!("{:?}", arr4);
    println!("arr1[0]: {}, arr3[2]: {}", arr1[0], arr3[2]);
}

//-----------------------------------------------------
pub fn code2_7() {
    let score = 59;
    let username = "zhangsan";
    let mut student = Student {
        score,
        name: username,
    };
    student.score = 60;
    println!("name: {}, score: {}", student.name, student.score);
    let student2 = Student {
        name: "lisi",
        ..student
    };
    println!("name: {}, score: {}", student2.name, student2.score);
}

struct Student {
    name: &'static str,
    score: i32,
}

//-----------------------------------------------------
#[derive(Debug)]
enum ColorNoParam {
    Red,
    Yellow,
    Blue,
}

pub fn code2_8() {
    let color_no_param = ColorNoParam::Red;
    match color_no_param {
        ColorNoParam::Red => println!("{:?}", ColorNoParam::Red),
        ColorNoParam::Yellow => println!("{:?}", ColorNoParam::Yellow),
        ColorNoParam::Blue => println!("{:?}", ColorNoParam::Blue),
    }
}

//-----------------------------------------------------
#[derive(Debug)]
enum ColorParam {
    Red(String),
    Yellow(String),
    Blue(String),
}

pub fn code2_9() {
    println!("{:?}", ColorParam::Blue(String::from("blue")));
}

//-----------------------------------------------------
pub fn code2_10() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("v: {:?}", v);
}

//-----------------------------------------------------
pub fn code2_11() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v[1] = 5;
    println!("v: {:?}", v);
}

//-----------------------------------------------------
pub fn code2_12() {
    let mut v: Vec<i32> = Vec::with_capacity(10);
    v.push(1);
    v.push(2);
    v.push(3);
    println!("e: {:?}", v.pop());
    println!("v: {:?}", v);
}

//-----------------------------------------------------
pub fn code2_13() {
    let mut v = vec![1, 2, 3];
    println!("e: {}", v.remove(1));
    println!("v: {:?}", v);
}

//-----------------------------------------------------
pub fn code2_14() {
    let v = vec![1, 2, 3];
    println!("e: {}", v[2]);
    // println!("e: {}", v[10]);
}
//-----------------------------------------------------

pub fn code2_15() {
    let v = vec![1, 2, 3];
    println!("e: {:?}", v.get(2));
    println!("e: {:?}", v.get(10));
}

//-----------------------------------------------------
pub fn code2_16() {
    let v = vec![10, 20, 30];
    for i in v {
        print!("{} ", i);
    }
    let mut v = vec![10, 20, 30];
    for i in &mut v {
        *i += 50;
        print!("{} ", i);
    }
}

//-----------------------------------------------------
pub fn code2_17() {
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_front(1);
    v.push_front(2);
    println!("v: {:?}", v);
}

//-----------------------------------------------------
pub fn code2_18() {
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v[1] = 5;
    println!("v: {:?}", v);
}

//-----------------------------------------------------
pub fn code2_19() {
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    v.push_front(1);
    v.push_front(2);
    println!("e: {:?}", v.pop_back());
    println!("e: {:?}", v.pop_front());
    println!("v: {:?}", v);
}

//-----------------------------------------------------
pub fn code2_20() {
    let mut v: VecDeque<u32> = VecDeque::with_capacity(10);
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    println!("e: {:?}", v.remove(1));
    println!("e: {:?}", v.remove(5));
    println!("v: {:?}", v);
}

//-----------------------------------------------------
pub fn code2_21() {
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    println!("e: {}", v[0]);
    // println!("e: {}", v[10]);
}

//-----------------------------------------------------
pub fn code2_22() {
    let mut v: VecDeque<u32> = VecDeque::new();
    v.push_back(1);
    v.push_back(2);
    v.push_back(3);
    println!("e: {:?}", v.get(0));
    println!("e: {:?}", v.get(10));
}

//-----------------------------------------------------
pub fn code2_23() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    let zhangsan1 = map.insert("zhangsan", 97);
    map.insert("lisi", 86);
    println!("{:?}", zhangsan1);
    println!("{:?}", map);
    let zhangsan2 = map.insert("zhangsan", 79);
    println!("{:?}", zhangsan2);
    println!("{:?}", map);
}

//-----------------------------------------------------
pub fn code2_24() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.entry("zhangsan").or_insert(97);
    map.entry("lisi").or_insert(86);
    println!("{:?}", map);
    map.entry("zhangsan").or_insert(79);
    println!("{:?}", map);
}

//-----------------------------------------------------
pub fn code2_25() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("zhangsan", 97);
    map.insert("lisi", 86);
    map.insert("wangwu", 55);
    println!("{:?}", map);
    for (_, val) in map.iter_mut() {
        *val += 2;
    }
    println!("{:?}", map);
}

//-----------------------------------------------------
pub fn code2_26() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("zhangsan", 97);
    map.insert("lisi", 86);
    map.insert("wangwu", 55);
    println!("{:?}", map);
    let result = map.remove("wangwu");
    println!("{:?}", map);
    println!("{:?}", result);
}

//-----------------------------------------------------
pub fn code2_27() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("zhangsan", 97);
    println!("zhangsan: {}", map["zhangsan"]);
    // println!("wangwu: {}", map["wangwu"]);
}

//-----------------------------------------------------
pub fn code2_28() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("zhangsan", 97);
    println!("zhangsan: {:?}", map.get("zhangsan"));
    println!("wangwu: {:?}", map.get("wangwu"));
}

//-----------------------------------------------------
pub fn code2_29() {
    let mut s = String::from("Hello, ");
    s.push('R');
    s.push_str("ust!");
    println!("{}", s);
}

//-----------------------------------------------------
pub fn code2_30() {
    let mut s = String::from("Hello World!");
    s.insert(5, ',');
    s.insert_str(7, "Rust ");
    println!("{}", s);
}

//-----------------------------------------------------
pub fn code2_31() {
    let s1 = String::from("Hello");
    let s2 = String::from(", ");
    let s3 = String::from("Rust ");
    let s4 = "World";
    let mut s = s1 + &s2 + s3.as_str() + s4;
    s += "!";
    println!("{}", s);
}

//-----------------------------------------------------
pub fn code2_32() {
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = "World";
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

//-----------------------------------------------------
pub fn code2_33() {
    let s = String::from("aaabbbbccaadd");
    let s1 = s.replace("aa", "77");
    let s2 = s.replacen("aa", "77", 1);
    println!("{}", s1);
    println!("{}", s2);
}

//-----------------------------------------------------
pub fn code2_34() {
    let mut s = String::from("Löwe 老虎 Léopard");
    println!("{:?}", s.pop());
    println!("{}", s);
    println!("{:?}", s.remove(9));
    println!("{}", s);
    s.truncate(9);
    println!("{}", s);
    s.clear();
    println!("{}", s);
}

//-----------------------------------------------------
pub fn code2_35() {
    let s = String::from("Löwe 老虎");
    println!("Löwe 老虎: {}", s.len());
    let s = String::from("L");
    println!("L: {}", s.len());
    let s = String::from("ö");
    println!("ö: {}", s.len());
    let s = String::from("老");
    println!("老: {}", s.len());
}

//-----------------------------------------------------
pub fn code2_36() {
    let s = String::from("Löwe 老虎");
    let bytes = s.bytes();
    for b in bytes {
        print!("{} | ", b);
    }
    println!();
    let chars = s.chars();
    for c in chars {
        print!("{} | ", c);
    }
}
//-----------------------------------------------------
