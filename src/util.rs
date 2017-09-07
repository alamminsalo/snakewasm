
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

