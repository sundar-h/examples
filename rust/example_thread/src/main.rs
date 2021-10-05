fn main() {
    let a = [1, 2, 3];
    let b = a;
    println!("{:?} {:#?}", a, b);
}

// fn main() {
//     let a = [1, 2, 3];
//     let b = a;
//     {
//         ::std::io::_print(
//             match match (&a, &b) {
//                 (arg0, arg1) => [
//                     ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
//                     ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
//                 ],
//             } {
//                 ref args => unsafe { ::core::fmt::Arguments::new_v1(&["", " ", "\n"], args) },
//             },
//         );
//     };
// }
