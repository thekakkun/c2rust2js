use c2rust::add;
use c2rust::hello;

// struct Sys {
//     elems: Vec<bindings::Element>,
// }

// fn square(sys: &mut Sys) {
//     unsafe {
//         bindings::square_elems(bindings::System {
//             elem: sys.elems.as_mut_ptr(),
//             elem_count: sys.elems.len() as i32,
//         })
//     }
// }

fn main() {
    println!("{}", hello());
    println!("{}", add(43, 44));

    //     let mut sys = Sys {
    //         elems: vec![
    //             bindings::Element { val: 1 },
    //             bindings::Element { val: 2 },
    //             bindings::Element { val: 3 },
    //         ],
    //     };

    //     println!("{:?}", sys.elems[1]);
    //     square(&mut sys);
    //     println!("{:?}", sys.elems[1]);
}
