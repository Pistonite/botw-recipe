
/// Precomputed data for (n multichoose k) where 0 <= n < N and 0 <= k < K
pub struct Multichoose<const N: usize, const K: usize> {
    /// data[i][m] = m multichoose i
    data: [[u64; N]; K],
}
