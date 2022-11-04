use std::collections::HashMap;
use std::fmt::{Debug, Display};


//--------------------------------------------------
pub fn code6_1() {
    let x = 5;
    let y = x;
    println!("x:{},y:{}", x, y);
    let _s1 = String::from("hello");
    //let s2 = s1;
    //println!("s1:{},s2:{}", s1, s2);
}

//--------------------------------------------------
pub fn code6_2() {
    let s = String::from("hello");
    take_ownership(s);
    let x = 5;
    make_copy(x);
}

fn take_ownership(str: String) {
    println!("{}", str);
}

fn make_copy(int: i32) {
    println!("{int}");
}

//--------------------------------------------------
pub fn code6_3() {
    let key = "favorit color";
    let value = "Red";
    let mut map = HashMap::new();
    map.insert(key, value);
    println!("{}", map[key]);
}

//--------------------------------------------------
pub fn code6_4() {
    let key = String::from("favorite color");
    let value = String::from("Red");
    let mut map = HashMap::new();
    map.insert(key, value);
    //println!("{}", map[key]);
}
//--------------------------------------------------

pub fn code6_5() {
    let key = String::from("favorite color");
    let value = String::from("Red");
    let mut map = HashMap::new();
    map.insert(&key, &value);
    println!("{}", map[&key]);
}

//--------------------------------------------------
pub fn code6_6() {
    let s1 = give_ownership();
    let s2 = take_and_give_back(s1);
    println!("{s2}");
}

fn give_ownership() -> String {
    let str = String::from("ownership");
    str
}

fn take_and_give_back(name: String) -> String {
    let hello = String::from("hello");
    hello + " " + &name
}

//--------------------------------------------------
#[derive(Debug, Copy, Clone)]
struct Foo {
    x: i32,
    y: bool,
}

pub fn code6_7() {
    let foo = Foo {
        x: 8,
        y: true,
    };
    let other = foo;
    println!("foo:{:?},other:{:?}", foo, other);
}

//--------------------------------------------------
#[derive(Debug, Clone)]
struct Foo1 {
    x: i32,
    y: String,
}

pub fn code6_8() {
    let foo = Foo1 {
        x: 8,
        y: String::from("hello"),
    };
    //let other = foo;
    //println!("foo:{:?},other:{:?}", foo, other);
}

//--------------------------------------------------
pub fn code6_9() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

//--------------------------------------------------
fn sum_vec(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    v1.iter().sum::<i32>() + v2.iter().sum::<i32>()
}

pub fn code6_10() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let answer = sum_vec(vec1, vec2);
    //println!("v1: {:?}, v2: {:?}, sum: {}", vec1, vec2, answer);
}

//--------------------------------------------------
pub fn code6_11() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let (v1, v2, answer) = sum_vec1(vec1, vec2);
    println!("v1: {:?}, v2: {:?}, sum: {}", v1, v2, answer);
}

fn sum_vec1(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    let sum = v1.iter().sum::<i32>() + v2.iter().sum::<i32>();
    (v1, v2, sum)
}

//--------------------------------------------------
pub fn code6_12() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let answer = sum_vec2(&vec1, &vec2);
    println!("v1: {:?}, v2: {:?}, sum: {}", vec1, vec2, answer);
}

fn sum_vec2(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    v1.iter().sum::<i32>() + v2.iter().sum::<i32>()
}

//--------------------------------------------------
pub fn code6_13() {
    let mut vec = Vec::new();
    push_vec(&mut vec, 1);
    push_vec(&mut vec, 2);
    push_vec(&mut vec, 3);
    push_vec(&mut vec, 4);
    push_vec(&mut vec, 5);
    println!("vec:{:#?}", vec);
}

fn push_vec(v: &mut Vec<i32>, value: i32) {
    if v.is_empty() || v.get(v.len() - 1).unwrap() < &value {
        v.push(value);
    }
}

//--------------------------------------------------
pub fn code6_14() {
    let mut x = 6;
    let y = &mut x;
    *y += 1;
    println!("x:{x}");
}

//--------------------------------------------------
pub fn code6_15() {
    let mut x = 6;
    let y = &mut x;
    *y += 1;
    //let z = &x;
    //println!("y:{},z:{}", *y, *z);
}

//--------------------------------------------------
pub fn code6_16() {
    let mut x = 6;
    let y = &mut x;
    *y += 1;
    //let z = x;
    //println!("y:{},z:{}", *y, z);
}

//--------------------------------------------------
pub fn code6_17() {
    let s = String::from("Hello, Rust!");
    println!("{}", &s[0..5]);
    println!("{}", &s[..5]);
    println!("{}", &s[7..s.len()]);
    println!("{}", &s[7..]);
    println!("{}", &s[0..s.len()]);
    println!("{}", &s[..]);
    let vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", &vec[0..2]);
    println!("{:?}", &vec[..2]);
    println!("{:?}", &vec[2..vec.len()]);
    println!("{:?}", &vec[2..]);
    println!("{:?}", &vec[0..vec.len()]);
    println!("{:?}", &vec[..]);
}

//--------------------------------------------------
pub fn code6_18() {
    let s = String::from("Hello, Rust!");
    let str = "Hello";
    let vec = vec![1, 2, 3, 4, 5];
    print_str(&s[0..5]);
    print_str(&str);
    print_vec(&vec[2..]);
}

fn print_str(s: &str) {
    println!("slice:{:?},length:{}", s, s.len());
}

fn print_vec(vec: &[i32]) {
    println!("slice:{:?},length:{}", vec, vec.len());
}

//--------------------------------------------------
pub fn code6_19() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let vec_slice = &mut vec[3..];
    vec_slice[0] = 7;
    println!("{vec:?}");
}

//--------------------------------------------------
pub fn code6_20() {
    let vec = vec!["Java", "Rust", "Python"];
    for str in vec.into_iter() {
        match str {
            "Rust" => println!("niubility"),
            _ => println!("{str}")
        }
    }
    //println!("{:?}", vec);
}

//--------------------------------------------------
pub fn code6_21() {
    let vec = vec!["Java", "Rust", "Python"];
    for str in vec.iter() {
        match str {
            &"Rust" => println!("niubility"),
            _ => println!("{str}")
        }
    }
    println!("{vec:?}");
}

//--------------------------------------------------
pub fn code6_22() {
    let mut vec = vec!["Java", "Rust", "Python"];
    for str in vec.iter_mut() {
        match str {
            &mut "Rust" => {
                *str = "niubility";
                println!("{str}");
            }
            _ => println!("{str}")
        }
    }
    println!("{vec:?}");
}

//--------------------------------------------------
pub fn code6_23() {
    let r;
    {
        let i = 7;
        r = &i;
    }
    //println!("r:{r}");
}

//--------------------------------------------------
pub fn code6_24() {
    let r;//---------------------------
    {
        let i = 7;  //------------
        r = &i;         //          'b    'a
    }//--------------------------------
    //println!("r:{r}");//---------------------
}

//--------------------------------------------------
pub fn code6_25() {
    let i = 7;//---------------------'b
    let r = &i;//--------------'a
    println!("r:{r}");
}

//--------------------------------------------------
pub fn code6_26() {
    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = long_str(str1.as_str(), str2);
    println!("longer string:{result}");
}

fn long_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//--------------------------------------------------
pub fn code6_27() {
    let str1 = String::from("abcd");
    let result;
    {
        let str2 = String::from("xyz");
        result = long_str(str1.as_str(), str2.as_str());
    }
    //println!("longer string:{result}");
}

//--------------------------------------------------
pub fn code6_28() {
    let str1 = String::from("abcd");
    let result;
    {
        let str2 = "xyz";
        result = long_str(str1.as_str(), str2);
    }
    println!("longer string:{result}");
}

//--------------------------------------------------
pub fn code6_29() {
    let str = long_str_with_tip("abcdefg", "xyz", 123);
    println!("{str}");
}

fn long_str_with_tip<'a, T>(x: &'a str, y: &'a str, tip: T) -> &'a str
    where T: Display {
    println!("Tip:{tip}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//--------------------------------------------------
pub fn code6_30() {
    let f1 = Foo2 {
        x: &3,
        y: &5,
    };
    let f2 = Foo2 {
        x: &7,
        y: &9,
    };
    println!("x:{},y:{}", f1.get_x(), f1.get_y());
    println!("max_x:{}", f1.max_x(&f2));
}

struct Foo2<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

impl<'a, 'b> Foo2<'a, 'b> {
    fn get_x(&self) -> &i32 {
        self.x
    }
    fn get_y(&self) -> &i32 {
        self.y
    }
    fn max_x(&'a self, f: &'a Foo2) -> &'a i32 {
        if self.x > f.x {
            self.x
        } else {
            f.x
        }
    }
}
//--------------------------------------------------
