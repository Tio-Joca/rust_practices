use rust_practices::calculate_length_using_reference;
use rust_practices::change;
use rust_practices::undo_change;
use rust_practices::no_dangle;

fn main () {
    //  References and borrowing
    //  Previously, we talked about ownership and ownership transfer
    //  process. But, if you remember, when we have to use some
    //  complex data allocated in heap and managed by some variable
    //  as function argument to a function call, a ownership transfer
    //  occurs. So, if we want to use this heap allocated data, we
    //  need to make a ownership transfer in the return of the
    //  function to the caller. However, Rust provides for us a way
    //  to use these kinds of heap allocated data in a function call
    //  without the trouble of ownership transfer between the owner
    //  and the function call. To make this possible, we have the
    //  references at our service. References provides a resource to
    //  borrow the respective heap allocated data instead of need to
    //  use ownership transfers.

    //  Usage of immutable reference in function call
    let s1: String = String::from("Hello");
    let length: usize = calculate_length_using_reference(&s1);

    println!("The length of the String managed by s1 variable is {}.", length);

    //  Ownership transfer between s1 and s2
    let mut s2: String = s1;

    //  Output of the data in s2 variable before the function call
    println!("The original String controlled by s2 is '{}'.", s2);

    //  Usage of mutable reference in function call
    change(&mut s2);

    //  Output of the data in s2 variable after the function call
    println!("The updated String controlled by s2 is '{}'.", s2);

    //  The rules of references
    //  While references is a convenient resource to work with heap
    //  allocated data without ownership transfers, you'll have to
    //  follow some rules. These rules are listed below:
    //  1) At any given time, you can have either one mutable
    //  reference or any number of immutable references;
    //  2) References must always be valid.
    //  Last but not least, we need to talk about the scope of
    //  references. Differently from standard variables, the scope
    //  of a reference begins when it's initialiazed and ends when
    //  it's used for the last time in your source code.

    //  Caution!!!
    //  If you try to remove these lines of code from comments and
    //  try to compile, you'll get a compile-time error.
    //  let mut r1: &mut String = &mut s2;
    //  let mut r2: &mut String = &mut s2;
    //  undo_change(r1);
    //  change(r2);
    //  This error occurs because the first two lines declare and
    //  initialize variables that will receive mutable references
    //  to the same data in heap and the first reference scope
    //  collides with the second reference scope. This collision
    //  occurs because, when the mutable reference in r1 is used
    //  in undo_change, the mutable reference in r2 exists at the
    //  same time.

    //  Now, I'll demonstrate two ways to avoid the previous example
    //  of compile-time error

    //  First example
    let mut r1: &mut String;
    let mut r2: &mut String;

    r1 = &mut s2;

    undo_change(r1);

    //  This output was made to provide a visualization of the String
    //  after the undo_change function call.
    println!("The updated String value controlled by s2 after undo_change() is '{}'.", r1);

    r2 = &mut s2;

    change(r2);

    //  This output was made to provide a visualization of the String
    //  after the change function call.
    println!("The updated String value controlled by s2 after change() is '{}'.", r2);

    //  Second example
    {
        let mut r1: &mut String = &mut s2;

        undo_change(r1);

        //  This output was made to provide a visualization of the
        //  String after the undo_change function call.
        println!("The updated String value controlled by s2 after undo_change() is '{}'.", r1);
    }

    let mut r2: &mut String = &mut s2;

    change(r2);

    //  This output was made to provide a visualization of the String
    //  after the change function call.
    println!("The updated String value controlled by s2 after change() is '{}'.", r2);

    //  Caution!!!
    //  If you try to remove these lines of code from comments and
    //  try to compile, you'll get a compile-time error.
    //  let ref1: &String = &s2;
    //  let ref2: &String = &s2;
    //  let mut ref3: &mut String = &mut s2;
    //  println!("The length of the String controlled by s2 using calculate_length_using_reference(ref1): {}.\nThe length of the String controlled by s2 using calculate_length_using_reference(ref2): {}.", calculate_length_using_reference(ref1), calculate_length_using_reference(ref2));
    //  undo_change(ref3);
    //  println!("The updated String controlled by s2 after undo_change(ref3): '{}'.", s2);
    //  This error occurs because the third line of code declares and 
    //  initializes a varible with a mutable reference to the same
    //  heap allocated data refered by the two immutable references
    //  controlled by the previous declared and initialized variables,
    //  resulting in a reference scope collision between the mutable
    //  reference and the two immutable references. This collision
    //  occurs because, when the ref1 and ref2 are used by calculate_
    //  length_using_reference() inside the println!(), the mutable
    //  reference in ref3 exists at the same time.

    //  Now, I'll demonstrate a method to avoid the previous example
    //  of compile-time error

    let ref1: &String;
    let ref2: &String;
    let mut ref3: &mut String;

    ref1 = &s2;
    ref2 = ref1;

    println!("The length of the String controlled by s2 using calculate_length_using_reference(ref1): {}.\nThe length of the String controlled by s2 using calculate_length_using_reference(ref2): {}.", calculate_length_using_reference(ref1), calculate_length_using_reference(ref2));

    ref3 = &mut s2;

    undo_change(ref3);

    println!("The updated String controlled by s2 after undo_change(ref3): '{}'.", s2);

    //  Dangling references
    //  If you make a reference to a heap allocated data and this data
    //  was dropped when it goes out of scope, you'll have a dangling
    //  reference. This issue will generate a compile-time error,
    //  because the rust compiler won't allow neither mutable or
    //  immutable references to memory addresses in heap that don't
    //  contain the respective data referenced by those references.

    //  The function used in the variable declaration and assignment
    //  code in the line of comment below will cause this kind of
    //  compile-time error.
    //  let reference_to_nothing: &String = dangle();
    //  The comments outside the scope of the main function contain the
    //  function responsible for the error. If you want to see the error
    //  in practice, remove those codes from comments and try to compile.

    //  Instead of try to return a reference to the String allocated in
    //  heap, we can make a ownership transfer to the caller. This
    //  approach was implemented in the no_dangle function, located in
    //  lib.rs source code.

    let words: String = no_dangle();

    println!("The String data controlled by words is '{}'.", words);
}

//  fn dangle () -> &String {
//      let s: String = String::from("Rust is funny");
//      return &s;
//  }
