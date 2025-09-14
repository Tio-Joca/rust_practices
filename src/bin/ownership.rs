use rust_practices::gives_ownership;
use rust_practices::takes_ownership;
use rust_practices::makes_copy;
use rust_practices::takes_and_gives_back;
use rust_practices::calculate_length;

fn main () {
    //  Scope of variables
    //  Each variable has a scope where is valid to be used in some
    //  program. The scope is delimited by the curly brackets and
    //  the declaration statement where the variable is located. It
    //  remains valid until it goes out of scope.

    //  The variable s isn't valid until its declaration occurs
    let s: &str = "Hello";
    //  Now, s is valid until it goes out of scope, represented by
    //  the curly bracket at the end of this block of code.
    println!("{}. This hello is from a string literal stored in the s variable.", s);

    //  Ownership
    //  The data used by programs can be stored in stack or heap. In
    //  Rust, values from primitive types managed by variables are
    //  located in stack and values from complex types like String
    //  are located in heap. Because of the fact about the risks and
    //  challenges involving handling memory in heap, Rust implements
    //  some techniques to provide memory safety with performance at
    //  low level. One of these techniques are is the Ownership.
    //  Ownership provides for us the guarantee to use the heap
    //  without problems like two or more pointers to the same data in
    //  heap. Ownership works based in three rules:
    //  1) Each value in Rust has an owner;
    //  2) There can only be one owner at a time;
    //  3) When the owner goes out of scope, the value will be dropped.

    //  This s variable is the owner of the String value allocated in
    //  heap in initialization.
    let s: String = String::from("Hello");

    println!("{}. This hello is from a String stored in the s variable, but the data was allocated in heap.\nThe previous hello stored by the s variable was allocated in stack.", s);

    //  It's possible that some mutable variable can be the owner of
    //  some value located in heap. The example below demonstrates
    //  this possibility.
    let mut s: String = String::from("Hello");
    println!("{}. This hello is from a String stored in the s variable with its data allocated in heap, but this s variable is mutable.", s);

    s.push_str(" world!");

    println!("{}", s);

    //  Ownership transfer
    let s1: String = String::from("Hello world!");
    let s2: String = s1;

    //  If you change the println! macro below to use the s1 variable
    //  and try to compile this source code, you'll get a compile-time
    //  error. This error occurs because the ownership related to the
    //  String value was transfered from s1 variable to s2 variable,
    //  what means s1 doesn't have a String related because it doesn't
    //  have ownership to a String value.
    println!("{}", s2);

    //  Caution!!!
    //  Ownership transfer only happens using complex data that will be
    //  allocated in heap. If you try to make an variable assignment to
    //  a variable related to a primitive type using other variable
    //  related to the same type and previously initialized, the variable
    //  will receive a copy of the value managed by the variable used to
    //  get the value.

    //  If a ownership transfer occurs here, the code in line 69 will
    //  generate a compile-time error. But, in this case, the value of x
    //  variable is copied to be assigned to the y variable.
    let x: i64 = 5;
    let y: i64 = x;

    println!("The value of x is {}.", x);
    println!("The value of y is {}.", y);

    //  Copy of data allocated in heap
    //  If you want to do a copy of heap allocated data, just like the
    //  previous example do for primitive types, you will need to use the
    //  the clone method provided by Rust.

    let s1: String = String::from("Hello world");
    let s2: String = s1.clone();

    println!("{} using s1 variable.", s1);
    println!("{} using s2 variable.", s2);

    //  Ownership and functions
    //  In Rust, the usage of values and variables as arguments to some
    //  function call works just like the assignments described above.
    //  Usually, primitive values are copied to be used by the function
    //  and ownerships related to complex kinds of data are transfered
    //  to the function scope.
    let integer: i32 = 23;
    let mut words: String = String::from("Ownership");

    takes_ownership(words);
    makes_copy(integer);

    //  Because of ownership transfer done in line 97, if you try to
    //  compile the line of code below, you'll get an compile-time error.
    //  println!("The value stored by the word variable is {}.", words);
    println!("The value {} previously stored in the integer variable is still intact.", integer);

    //  Return values and scope
    //  While we have to deal with complex data and the ownership associated
    //  with this data to use it with some function taking the ownership
    //  to the function scope, we can make a ownership transfer from the
    //  function scope to the function caller. Furthermore, we can return
    //  multiple values return them with a tuple.

    //  Ownership transfer from the gives_ownership function to the words
    //  variable.
    words = gives_ownership();
    //  Now, words have an ownership to a String data created by this
    //  function.

    println!("{}", words);

    //  In this example, the ownership was transfered from words to the
    //  takes_and_gives_back function. But, the function returns the
    //  ownership to words.
    words = takes_and_gives_back(words);

    println!("{}\nAs you can see, the ownership to the heap allocated data was returned to the words variable.", words);

    let length: usize;

    (words, length) = calculate_length(words);
    
    println!("{}\nThe length of the previous phrase is {}.", words, length);
}
