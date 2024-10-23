use std::thread::spawn;

fn hello() {
    println!("Hello, world!");
}

fn my_func() {
    spawn(hello).join().unwrap();
    
    let h = || println!("Hello, world!");
    spawn(h).join().unwrap();
}

fn main() {
    // my_func();

    let v = 10;
    let f = move || v * 2;

    let result = spawn(f).join();
    println!("result = {:?}", result);

    match spawn(|| panic!("Paniced!")).join() {
        Ok(_) => println!("successed"),
        Err(e) => {
            let s = e.downcast_ref::<&str>();
            println!("failed: {:?}", s);
        }
    }
}