//-----------------------------------------------------
pub fn code5_1() {
    let mut vec_integer = vec![10, 20];
    vec_integer.push(30);
    println!("{vec_integer:?}")
}

//-----------------------------------------------------
struct Rectangle1<T> {
    width: T,
    height: T,
}

struct Rectangle2<T, U> {
    width: T,
    height: U,
}

impl<T> Rectangle1<T> {
    fn width(&self) -> &T {
        &self.width
    }
    fn height(&self) -> &T {
        &self.height
    }
}

impl Rectangle1<i32> {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl<T, U> Rectangle2<T, U> {
    fn width(&self) -> &T {
        &self.width
    }
    fn height(&self) -> &U {
        &self.height
    }
}

pub fn code5_2() {
    let rect1 = Rectangle1 { width: 8, height: 2 };
    println!("rect1.width: {}, rect1.height: {}", rect1.width(), rect1.height());
    println!("rect1.area: {}", rect1.area());
    let rect2 = Rectangle2 { width: 8, height: 2.2 };
    println!("rect2.width: {}, rect2.height: {}", rect2.width(), rect2.height())
}

//-----------------------------------------------------
fn option_add(x: Option<i32>, y: Option<i32>) -> Option<i32> {
    return if x.is_none() && y.is_none() {
        None
    } else if x.is_none() {
        y
    } else if y.is_none() {
        x
    } else {
        Some(x.unwrap() + y.unwrap())
    };
}

fn option_print(opt: Option<i32>) {
    match opt {
        Some(result) => println!("Option:{result}"),
        _ => println!("Option is None!"),
    }
}

pub fn code5_3() {
    let result1 = option_add(Some(3), Some(5));
    let result2 = option_add(Some(3), None);
    let result3 = option_add(None, None);
    option_print(result1);
    option_print(result2);
    option_print(result3);
}

//-----------------------------------------------------
fn foo<T>(x: T) -> T {
    x
}

pub fn code5_4() {
    println!("{}", foo(5));
    println!("{}", foo("hello"));
}

//-----------------------------------------------------
trait Geometry {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Geometry for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        (self.width + self.height) * 2.0
    }
}

struct Circle {
    radius: f32,
}

impl Geometry for Circle {
    fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        3.14 * 2.0 * self.radius
    }
}

pub fn code5_5() {
    let rect = Rectangle {
        width: 8.8,
        height: 2.2,
    };
    println!("rect.area: {}, rect.perimeter: {}", rect.area(), rect.perimeter());
    let circle = Circle {
        radius: 3.0
    };
    println!("circle.area: {}, circle.perimeter: {}", circle.area(), circle.perimeter());
}

//-----------------------------------------------------
fn print(geometry: impl Geometry) {
    println!("area: {}, perimeter: {}", geometry.area(), geometry.perimeter());
}

pub fn code5_6() {
    let rect = Rectangle { width: 10.5, height: 5.5 };
    print(rect);
}


//-----------------------------------------------------
use std::fmt::{Display, Formatter, Result};

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Rectangle:({}, {})", self.width, self.height)
    }
}

fn print1(geometry: impl Geometry + Display) {
    println!("{}, area: {}, perimeter: {}", geometry, geometry.area(), geometry.perimeter());
}

pub fn code5_7() {
    let rect = Rectangle {
        width: 10.5,
        height: 5.5,
    };
    print1(rect);
}

//-----------------------------------------------------
fn area_add(geo1: impl Geometry, geo2: impl Geometry) {
    println!("rect.area:{},circle.area:{},total atea:{}", geo1.area(), geo2.area(), geo1.area() + geo2.area());
}

pub fn code5_8() {
    let rect = Rectangle {
        width: 10.5,
        height: 5.5,
    };
    let circle = Circle {
        radius: 3.0
    };
    area_add(rect, circle);
}

//-----------------------------------------------------
impl Display for Circle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "/circle:({})", self.radius)
    }
}

fn print2<T: Geometry + Display>(geometry: T) {
    println!("{},area:{},perimeter:{}", geometry, geometry.area(), geometry.perimeter());
}

pub fn code5_9() {
    let circle = Circle {
        radius: 3.0
    };
    print2(circle);
}

//-----------------------------------------------------
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub fn code5_10() {
    let origin = Point {
        x: 0,
        y: 0,
    };
    println!("{origin}");
    println!("{origin:?}");
    println!("{origin:#?}");
}

//-----------------------------------------------------
enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}

pub fn code5_11() {
    let b1 = Book {
        isbn: 3,
        format: BookFormat::Paperback,
    };
    let b2 = Book {
        isbn: 3,
        format: BookFormat::Ebook,
    };
    let b3 = Book {
        isbn: 5,
        format: BookFormat::Hardback,
    };
    assert!(b1 == b2);
    assert!(b1 != b3);
}

//-----------------------------------------------------
use std::cmp::Ordering;

#[derive(Eq)]
struct Person {
    id: u32,
    name: String,
    height: u32,
}

impl PartialEq<Self> for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

impl PartialOrd<Self> for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

pub fn code5_12() {
    let person1 = Person {
        id: 1,
        name: "zhangsan".to_string(),
        height: 168,
    };
    let person2 = Person {
        id: 2,
        name: "lisi".to_string(),
        height: 175,
    };
    let person3 = Person {
        id: 3,
        name: "sanwu".to_string(),
        height: 180,
    };
    assert_eq!(person1 < person2, true);
    assert_eq!(person2 > person3, false);
    assert!(person1.lt(&person2));
    assert!(person3.gt(&person2));
    let tallest_person = person1.max(person2).max(person3);
    println!("id:{},name:{}", tallest_person.id, tallest_person.name);
}

//-----------------------------------------------------
#[derive(Default, Debug)]
struct MyStruct {
    foo: i32,
    bar: f32,
}

pub fn code5_13() {
    let options1: MyStruct = Default::default();
    let options2 = MyStruct {
        foo: 7,
        ..Default::default()
    };
    println!("options1:{:?}", options1);
    println!("options1:{:?}", options2);
}

//-----------------------------------------------------
pub fn code5_14() {
    let x: u16 = 7;
    let y = x as u32;
    println!("u16:{},u32:{}", x, y);
    let x: u32 = u32::MAX;
    let y = x as u16;
    println!("u32:{},u16:{}", x, y);
    let x = 65u8;
    let y = x as char;
    println!("u8:{},char:{}", x, y);
    let x = 'A';
    let y = x as u8;
    println!("char:{},u8:{}", x, y);
    let x = 7;
    let y = x as f64;
    println!("i32:{},f64:{}", x, y);
    let x = 7.7;
    let y = x as i32;
    println!("f64:{},i32:{}", x, y);
}

//-----------------------------------------------------
pub fn code5_15() {
    let x = 7;
    let y = x.to_string();
    println!("i32: {}, String: {}", x, y);
    let x = 7.7;
    let y = x.to_string();
    println!("f64: {}, String: {}", x, y);
    let x = String::from("7");
    let y = x.parse::<i32>().unwrap();
    println!("String: {}, i32: {}", x, y);
    let x = String::from("7.7");
    let x1: f64 = x.parse().unwrap();
    println!("String: {}, f64: {}", x, x1);
    let x = String::from("hello");
    let y = x.as_str();
    println!("{y}");
    let x = "hello";
    let y = x.to_string();
    println!("{y}");
}
//-----------------------------------------------------
