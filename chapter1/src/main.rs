use std::thread;

fn main() {
    // two_threads();
    // closure_thread();
    // closure_thread_get_value_back();
    scoped_threads();
}

fn scoped_threads() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| { 
            println!("length: {}", numbers.len());
        });
        s.spawn(|| { 
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}

fn closure_thread_get_value_back() {
    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        return (sum / len);
    });

    let average = t.join().unwrap();
    println!("The average is: {:?}", average);
}

fn closure_thread() {
    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    }).join().unwrap();
}

fn two_threads() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread!");

    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello from a thread!");
    let id = thread::current().id();
    println!("This is my thread id: {:?}", id);
}