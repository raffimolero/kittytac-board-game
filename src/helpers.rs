pub fn arr_2d_from_iter<T, const N: usize>(mut iter: impl Iterator<Item = T>) -> [[T; N]; N] {
    [(); N].map(|_| {
        [(); N].map(|_| {
            iter.next().expect(&format!(
                "Ran out of items in an iterator while trying to fill an {N} by {N} array."
            ))
        })
    })
}
