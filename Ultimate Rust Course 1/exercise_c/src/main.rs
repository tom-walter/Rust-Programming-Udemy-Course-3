// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]
use exercise_c::*;

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    // dot syntax
    print_difference(coords.0, coords.1);

    // tuple to array
    let coords_array: [f32; 2] = [coords.0, coords.1];
    print_array(coords_array);

    // array indexing
    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series[6]);

    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0)
}
