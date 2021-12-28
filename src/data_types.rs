// Floating point
/*
Length  |   Signed  | Unsigned
8-bit   |   i8      |   u8
16-bit  |   i16     |   u16
32-bit  |   i32     |   u32
64-bit  |   i64     |   u64
128-bit |   i128    |   u129
*/
fn dataTypes() {
    let x = 2.0; // f64
    let y: f32 = 2.0; // f32
                      //-----------------------------------------------

    // Numeric operations
    let sum = 1 + 2;
    let difference = 15.0 - 5.0;
    let multiplication = 2 * 2;
    let division = 10 / 2;
    let remainder = 10 % 2;

    // Bolean type
    let t = true;
    let f: bool = false;

    // Character type
    let c = 'z'; // rust reserva aspas simples para caracteres e duplas para strings

    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    // Array type
    let a = [1, 2, 3, 4, 5];
    let days = ["Monday, Tuesday, Wednesday"];
}
