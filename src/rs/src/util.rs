use ndarray::{s, Array, Array2, Axis};

// Returns 0..-1 matrix based on coordinate order on input.
// Smaller value is closer to tail of the body.
// Coordinates not present on input vec are set to 0.
pub fn add_weight_matrix(mut dest: Array2<f32>, input: &[(i16, i16)]) -> Array2<f32> {
    let shape = dest.shape();
    let w = shape[0];
    let h = shape[1];
    let div = w as f32 * h as f32;

    for (i, (x, y)) in input.into_iter().rev().skip(1).enumerate() {
        // calculate body weight value in 0..1
        let w = i as f32 / div;
        *dest.get_mut((*y as usize, *x as usize)).unwrap() = -w;
    }

    dest
}

// Rolls input 2D array with given coordinate offset
pub fn roll_2d(arr: Array2<f32>, offset: (i16, i16)) -> Array2<f32> {
    // use view to avoid copying values early
    let v = arr.view();

    // to components x and y
    let (x, y) = offset;
    let x = x as i32;
    let y = y as i32;

    // y-axis
    let y1 = v.slice(s![..-y, ..]).into_owned();
    let y2 = v.slice(s![-y.., ..]).into_owned();
    let y = ndarray::concatenate![Axis(0), y2, y1];

    // x-axis
    let x1 = y.t().slice(s![..-x, ..]).into_owned();
    let x2 = y.t().slice(s![-x.., ..]).into_owned();
    let x = ndarray::concatenate![Axis(0), x2, x1];

    // transpose back to original format and return copied array
    x.t().into_owned()
}

// Rotates array clockwise N times
pub fn rot90(mut arr: Array2<f32>, n: usize) -> Array2<f32> {
    for _ in 0..n {
        arr = arr.t().slice(s![.., ..; -1]).to_owned();
    }
    arr
}

// Creates 2d inverse centered distance grid, scaled between 0..1 according to the input width
// Resulting example (roughly):
// [[0.5, 0.75, 0.5],
//  [0.75, 1.0, 0.75],
//  [0.5, 0.75, 0.5]],
pub fn dist_2d(w: usize, h: usize) -> Array2<f32> {
    let wf = w as f32;
    let hf = h as f32;

    // center coordinates
    let cx = wf / 2.0;
    let cy = hf / 2.0;

    let x = (Array::range(0., wf, 1.) - cx.floor()).map(|x| x.abs());
    let y = (Array::range(0., hf, 1.) - cy.floor()).map(|x| x.abs());

    1.0 - (x + y.broadcast((w, h)).unwrap().t()) / wf
}

// Calculates coordinate distance
pub fn dist_coord(a: (i16, i16), b: (i16, i16)) -> (i16, i16) {
    (a.0 - b.0, a.1 - b.1)
}

// Clamps value between bounds
pub fn wrap(min: i16, val: i16, max: i16) -> i16 {
    if val < min {
        return max;
    }
    if val > max {
        return min;
    }
    val
}
