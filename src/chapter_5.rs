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
