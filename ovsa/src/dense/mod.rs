use ndarray::Array1;



fn superposition(array_vec: &[Array1<f32>]) -> Array1<f32> {
    let mut result = Array1::<f32>::zeros(array_vec[0].len());
    // todo: optimize
    for array in array_vec {
        result += array;
    }

    result
}


fn circular_convolution(a: &Array1<f32>, b: &Array1<f32>) -> Array1<f32> {
    let n = a.len();
    let mut result = Array1::<f32>::zeros(n);

    // todo: optimize with matmul and slices
    for i in 0..n {
        for j in 0..n {
            let k = (i + j) % n;
            result[k] += a[i] * b[j];
        }
    }

    result
}


fn cyclic_shift(array: &Array1<f32>, shift_by: isize) -> Array1<f32> {
    let n = array.len() as isize;
    let mut result = Array1::<f32>::zeros(array.len());

    for old_index in 0..n {
        let new_index = (old_index + shift_by).rem_euclid(n);
        result[new_index as usize] = array[old_index as usize];
    }

    result
}