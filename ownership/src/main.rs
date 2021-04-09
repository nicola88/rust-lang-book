fn main() {
    println!("OWNERSHIP");
    // MOVE: change of ownership (shallow copy)
    // CLONE: create a copy of the data (deep copy, default with fixed-size types)
    // BORROW: refer a value without being its owner
    // DROP: deallocate value and free memory
    // Only allow 1 mutable reference OR n immutable references to avoid data race

    // PRIMITIVE TYPES
    // - Allocated on stack (fixed-size)
    // - Assignment: always copied (new owner)
    // - Function arguments: passed by value (new owner)
    let x1 = "Hello";
    let x2 = x1;
    println!("x1={}, x2={}", x1, x2);

    let n1 = 12;
    let n2 = 23;
    add(n1, n2);
    println!("n1={}, n2={}", n1, n2);


    // COMPLEX TYPES
    // - Allocated on heap (variable size)
    // - Assignment: change owner (previous owner becomes invalid)
    // - Function arguments: MOVE if passed by value (new owner)
    let x1 = String::from("Ciao"); // MOVE
    let x2 = x1; 
    // x1 is no longer valid
    // println!("x1={}, x2={}", x1, x2); // Do not compile!
    println!("x2={}", x2);
    let x3 = x2.clone().to_uppercase(); // CLONE
    println!("x2={}, x3={}", x2, x3);

    let s1 = String::from("Hello");
    print(s1); // MOVE into function and DROP when function returns
    let s1 = printret(String::from("HelloRet")); // MOVE into function and back to s1 on return
    println!("{}", s1); // s1 is again the OWNER
    let s1 = String::from("HelloRef");
    printref(&s1); // BORROW to function...
    println!("{}", s1); // ...s1 is still the OWNER
    let s1_len = length(&s1); // length BORROWS s1
    println!("len({})={}", s1, s1_len);

    let s1 = String::from("ABCDE");
    let s2 = String::from("FGHIJ");
    let s12 = concat(&s1, &s2);
    println!("{}+{}={}", s1, s2, s12);

    let mut s1 = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let slice = &s1[..5];
    s1.clear();
    println!("{}", slice);
}

fn add(n1: i32, n2: i32) {
    println!("{}+{}={}", n1, n2, n1 + n2);
}

fn print(str: String) {
    println!("print>{}", str);
}

fn printref(str: &String) {
    println!("printref>{}", str);
}

fn printret(str: String) -> String {
    println!("printret>{}", str);
    str
}

fn length(str: &String) -> usize {
    str.len()
}

fn concat(v1: &String, v2: &String) -> String {
    let mut tmp = String::from(v1);
    tmp.push_str(v2);
    tmp
}
