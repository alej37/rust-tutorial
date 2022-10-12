fn main() {
    println!("Hello, world!");
    let mut x:u32 = 4;
    println!("x is : {}", x);
    x = 5;
    println!("x is : {}", x);


    // potential risk as I am recreating a variable in memory with same name.
    let m = 3;
    println!("m is : {}", m);
    let m = 4;
    println!("m is : {}", m);

    // scope
     {
        // different scope out of layer outside in main
        // I can use a variable outside of this scope without it affecting the outside.
        let x = 2;
        println!("x is : {}", x);

     }

    //  constants
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("SECONDS: {}", SECONDS_IN_MINUTE);
    
    let tup:(i32, bool, char)= (1, true, 's');
    // tuples are inmutable by default, although that can be change by adding the mut keyword
    // check differece between char and &str, as it seems like type char is '' and &str is ""
    println!("{}, {}, {}",tup.0, tup.1, tup.2);

    let arr:[i32;4] = [1,2,3,4];
    println!("{}", arr[0])
    


}
