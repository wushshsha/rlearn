#[derive(Debug)]
struct Structure(i32);

//#[allow(dead_code)]
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age:u8
}

fn learn() {
    println!("{:?} months is a year.",12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's"
    );

    println!("Now {:?} will print!", Structure(2));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person{name:&name, age};
    println!("{:#?}",peter);
}
