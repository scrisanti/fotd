// mod enum_test;

// // ------ Variables ------
// fn main() {
//     let width1:u32 = 30;
//     let height1:u32 = 30;

//     println!(
//         "The are of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }


// // Area Function
// fn area(width: u32, height: u32) -> u32 {
//     width*height
// }

// // ------ Tuples ------
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The Area of th rectangel is {} sq px",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// ----- Struct ------
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     enum_test::main();

//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The are of the rectnagel is {} sq. px.",
//         area(&rect1)
//     )
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let data = "initial contents";
    print_type_of(&data); // &str

    let s = data.to_string();
    print_type_of(&s); // &str

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    
    let mut mystr = String::from("farts");
    mystr.push_str("AreStinky");
    print_type_of(&mystr);
    println!("{}", mystr)

}