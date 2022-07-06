fn main() {
    let x = 6;
    println!("{}", x);
    println!("{:p}", &x);

    let x = x + 1;
    println!("{}", x);
    println!("{:p}", &x);

    {
        let x = x + 1;
        println!("{}", x);
        println!("{:p}", &x);
    }

    println!("{}", x);
    println!("{:p}", &x);
    
}
