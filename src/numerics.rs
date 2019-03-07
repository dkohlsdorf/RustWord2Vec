
pub fn euclidean(x: &[f64], y: &[f64]) -> f64 {
    let mut distance = 0.0;
    for i in 0 .. x.len() {
        distance += (x[i] - y[i]).powf(2.0)
    }
    f64::sqrt(distance)
}

pub fn sub(x: usize, y: usize) -> usize {
    if x >= y {
        x - y
    } else {
        0
    }
}

pub fn dot(x: &[f64], y: &[f64]) -> f64 {
    let mut result = 0.0;
    for i in 0 .. x.len() {
        result += x[i] * y[i];
    }
    result
}

pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x))
}
