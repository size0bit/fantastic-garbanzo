use std::cell::RefCell;
use std::rc::Rc;

//-------------------------------------------------------
pub fn code7_1() {
    let x: Box<i32> = Box::new(5);
    let y: Box<i32> = x;
    //println!("{x}");
    println!("{y}");
}

//-------------------------------------------------------
pub fn code7_2() {
    let x: Box<i32> = Box::new(5);
    let y: i32 = *x;
    println!("x:{x}");
    println!("y:{y}");
}

//-------------------------------------------------------
pub fn code7_3() {
    let x: i32 = 5;
    let y: &i32 = &x;
    assert_eq!(5, *y);
    println!("pointer:{:p}", y);
}

//-------------------------------------------------------
pub fn code7_4() {
    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);
    assert_eq!(5, *y);
    println!("pointer:{:p}", y);
}

//-------------------------------------------------------
struct Custom {
    data: String,
}

impl Drop for Custom {
    fn drop(&mut self) {
        println!("Dropping Custom with data: {}", self.data);
    }
}

pub fn code7_5() {
    let str1 = Custom {
        data: "hello,world".to_string()
    };
    let str2 = Custom {
        data: "hello,rust".to_string()
    };
    println!("Custom created");
    println!("str1: {}", str1.data);
    println!("str2: {}", str2.data);
}

//-------------------------------------------------------
pub fn code7_6() {
    let x = Rc::new(5);
    println!("{:p},count after constructing x:{}", x, Rc::strong_count(&x));
    let y = x.clone();
    println!("{:p},count after constructing x:{}", x, Rc::strong_count(&x));
    {
        let z = Rc::clone(&x);
        println!("{:p},count after constructing z:{}", z, Rc::strong_count(&x));
    }
    println!("count after constructing z:{}", Rc::strong_count(&x));
}

//-------------------------------------------------------
pub fn code7_7() {
    let v = RefCell::new(vec![1, 2, 3, 4]);
    println!("{:?}", v.borrow());
    v.borrow_mut().push(5);
    println!("{:?}", v.borrow());
}

//-------------------------------------------------------
pub fn code7_8() {
    let v = RefCell::new(vec![1, 2, 3, 4]);
    let v_borrow_1 = v.borrow();
    println!("{v_borrow_1:?}");
    let v_borrow_2 = v.borrow();
    println!("{v_borrow_2:?}");
}

//-------------------------------------------------------
pub fn code7_9() {
    let v = RefCell::new(vec![1, 2, 3, 4]);
    let mut v_borrow_mut_1 = v.borrow_mut();
    v_borrow_mut_1.push(5);
    println!("{:?}", v_borrow_mut_1);
    // let mut v_borrow_mut_2 = v.borrow_mut();
    // v_borrow_mut_2.push(6);
    // println!("{:?}", v_borrow_mut_2);
}

//-------------------------------------------------------
pub fn code7_10() {
    let v = RefCell::new(vec![1, 2, 3, 4]);
    let v_borrow = v.borrow();
    println!("{:?}", v_borrow);
    // let mut v_borrow_mut = v.borrow_mut();
    // v_borrow_mut.push(5);
    // println!("{:?}", v_borrow_mut);
}
//-------------------------------------------------------
