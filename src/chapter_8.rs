use std::thread;
use std::time::Duration;
use async_std::task;
use futures::executor::block_on;

use threadpool::ThreadPool;


//--------------------------------------------------------------------------------------------------
pub fn code8_1() {
    let thread_1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_1 thread!", i);
            thread::sleep(Duration::from_secs(2));
        }
    });
    let thread_2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_2 thread!", i);
            thread::sleep(Duration::from_secs(2));
        }
    });
    for i in 1..=5 {
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(8));
    }
    thread_1.join().unwrap();
    thread_2.join().unwrap();
}

//--------------------------------------------------------------------------------------------------
pub fn code8_2() {
    let thread1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_1 thread!", i);
            thread::sleep(Duration::from_secs(3));
        }
    });
    let thread2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("number {} from the spawned_2 thread!", i);
            thread::sleep(Duration::from_secs(4));
        }
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
    for i in 1..=5 {
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(8));
    }
}

//--------------------------------------------------------------------------------------------------
pub fn code8_3() {
    let v = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(|| {
        //println!("{:?}", v);
    });
    handle.join().unwrap();
}

//--------------------------------------------------------------------------------------------------
pub fn code8_4() {
    let v = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });
    handle.join().unwrap();
}

//--------------------------------------------------------------------------------------------------
pub fn code8_5() {
    let pool = ThreadPool::new(3);
    for i in 1..=5 {
        pool.execute(move || {
            println!("number {} from the spawned_1 thread!", i);
        });
    }
    for i in 1..=5 {
        pool.execute(move || {
            println!("number {} from the spawned_2 thread!", i);
        });
    }
    for i in 1..=5 {
        println!("number {} from the main thread!", i);
    }
    pool.join();
}

//--------------------------------------------------------------------------------------------------
async fn hello_async() {
    println!("hello,async!");
}

pub fn code8_6() {
    let future = hello_async();
    block_on(future);
}

//--------------------------------------------------------------------------------------------------
// async fn learn_data_structure() -> DataStructure {}
//
// async fn learn_algorithm(data_structure: DataStructure) {}
//
// async fn learn_rust() {}
//
// pub fn code8_7() {
//     let data_structure = block_on(learn_data_structure());
//     block_on(learn_algorithm(data_structure));
//     block_on(learn_rust());
// }

//--------------------------------------------------------------------------------------------------
// async fn learn_data_structure_and_algorithm() {
//     let data_structure = learn_data_structure().await;
//     learn_algorithm(data_structure).await;
// }
//
// async fn async_main() {
//     let future1 = learn_data_structure_and_algorithm();
//     let future2 = learn_rust();
//
//     join!(future1, future2);
// }
//
// pub fn code8_8() {
//     block_on(async_main());
// }
//--------------------------------------------------------------------------------------------------
pub fn code8_9() {
    let async_1 = task::spawn(async {
        for i in 1..=5 {
            print_async_1(i).await;
        }
    });

    let async_2 = task::spawn(async {
        for i in 1..=5 {
            print_async_2(i).await;
        }
    });

    for i in 1..=5 {
        println!("number {} from the main!", i);
        task::block_on(async {
            task::sleep(Duration::from_secs(8)).await;
        });
    }

    task::block_on(async_1);
    task::block_on(async_2);
}

async fn print_async_1(i: i32) {
    println!("number {} from the async_1!", i);
    task::sleep(Duration::from_secs(2)).await;
}

async fn print_async_2(i: i32) {
    println!("number {} from the async_2!", i);
    task::sleep(Duration::from_secs(4)).await;
}
//--------------------------------------------------------------------------------------------------
