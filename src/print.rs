fn learn() {
    println!("hello world");
    let t = format!("hello");
    println!("HHH: {}", t);
    println!("A:{sdf},B:{d}", sdf = "df", d = 2);

    //二进制输出
    println!(
        "{} of {:b} people know a binary, the other half doesn't ",
        1, 2
    );
    println!("{number:>width$}", number = 123, width = 6);
    println!("{number:>0width$}", number = 123, width = 6);

    println!("My name is {0},{1}, {0}", "Bond", "James");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Structure(i32);
    println!("This struct {:?} won't print...", Structure(3));


    let pi = 3.141592;
    println!("Pi is roughly {0:.3}",pi);
}
