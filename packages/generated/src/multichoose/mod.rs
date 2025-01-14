//! Fast multichoose using pre-computation

mod gen;

/// Get multichoose(n, k) using pre-computed values.
/// n and k MUST be <= to MAX_N and MAX_K
unsafe fn multichoose(n: u32, k: u32) -> u64 {
    let x = unsafe {
        gen::MULTICHOOSE.get_unchecked(n as usize).get_unchecked(k as usize)
    };
    *x
}

/// A multichoose (combination with replacements)
/// series for choosing R objects from N objects, allowing for repetition.
///
/// It can convert between the serial ID (position of the choices in the
/// series of all choices) and the slice of choices
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mnr<const N: u32, const R: usize> {
    len: u64,
}
impl<const N: u32, const R: usize> Default for Mnr<N, R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: u32, const R: usize> Mnr<N, R> {
    /// Create a new multichoose series
    ///
    /// Returns None if N or R is too large
    pub fn new() -> Self {
        // exclude the trivial cases so we don't have to deal with them
        // in the algorithms
        if R >= (u32::MAX) as usize || R < 2 || (N as usize) < R || N > gen::MAX_N || R > (gen::MAX_K as usize) {
            // Compiler must optimize this out because of no_panic
            panic!("N or R is out of bounds");
        } else {
            // safety: checked
            let len = unsafe { multichoose(N, R as u32) };
            Self { len }
        }
    }

    /// Get number of elements in the series
    pub fn len(&self) -> u64 {
        self.len
    }

    /// Convert the serial ID to the choices.
    ///
    /// Returns false if the ID is out of bounds.
    ///
    /// # Time Complexity
    /// Currently the implementation is O(NR)
    #[must_use]
    pub fn serial_to_choices(&self, id: u64, out: &mut [u32; R]) -> bool {
        if id >= self.len {
            return false;
        }
        // id is the index into the set of all choices,
        // in the order of multichoose generation order

        // zero it just in case, (I don't remember if it's necessary but it's in the old impl)
        for o in out.iter_mut() {
            *o = 0;
        }

        // how many ids are left
        let mut rest = id;
        // how many items are left (since the inputs are ascending)
        let mut item_lower_bound = N;

        #[allow(non_snake_case)]
        let R_ = R as u32;

        #[allow(clippy::needless_range_loop)]
        for slot in 0..R_ {
            // compute the slot-th item in the input array
            let mut index = 0u64;
            for m in N + 1 - item_lower_bound..N + 1 {
                // does m overshot rest of the id
                // safety:
                // - 0 <= item_lower_bound <= N: see decl above and operation below
                // - which means 1 <= m <= N+1, 0 <= N+1-m <= N <= MAX_N
                // - 0 <= R - 1 - slot < R <= MAX_R
                let n_ = N + 1 - m;
                let k_ = R_ - 1 - slot;
                let next_block_size = unsafe { multichoose(n_, k_) };
                if index + next_block_size > rest {
                    // safety: 0 <= m - 1 < N
                    // so item will always be valid
                    out[slot as usize] = m - 1;
                    break;
                }
                index += next_block_size;
            }
            item_lower_bound = N - out[slot as usize];
            rest -= index;
        }

        true
    }

    /// Convert the choices to the serial ID.
    ///
    /// Returns None if any of the choices are out of bounds.
    ///
    /// The input must be sorted, otherwise the output is undefined.
    #[must_use]
    pub fn choices_to_serial(&self, choices: &[u32; R]) -> Option<u64> {
        // Bound check first
        for c in choices {
            if *c >= N {
                return None;
            }
        }
            
        let mut output = 0u64;
        // reconstruct rest_items to be at the beginning of last iteration
        let mut item_lower_bound = N - choices[R - 2];

        #[allow(non_snake_case)]
        let R_ = R as u32;

        // reverse the iterations
        for item in 0..R_ {
            // compute index
            // 0 <= reverse_item < R
            let reverse_item = R_ - 1 - item;
            let m = choices[reverse_item as usize] + 1;
            let mut index = 0u64;
            for reverse_m in N - item_lower_bound + 1..m {
                // safety:
                // - 1 <= item_lower_bound <= N
                // - m <= N
                // - 1 <= reverse_m <= N
                // - 1 <= N + 1 - reverse_m <= N <= MAX_N
                let n_ = N + 1 - reverse_m;
                index += unsafe { multichoose(n_, item) };
            }
            // add to output (reverse input -= index)
            output += index;
            // recover rest_items to beginning of last iteration
            if reverse_item > 1 {
                item_lower_bound = N - choices[reverse_item as usize - 2];
            } else {
                item_lower_bound = N;
            }
        }

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recipe_count_v1() {
        assert_eq!(Mnr::<183, 5>::new().len(), 1_805_568_402);
    }

    #[test]
    fn test_serial_to_choices() {
        let mnr = Mnr::<5, 3>::new();
        let mut out = [0; 3];
        assert_eq!(mnr.serial_to_choices(0, &mut out), true);
        assert_eq!(out, [0, 0, 0]);
        assert_eq!(mnr.serial_to_choices(3, &mut out), true);
        assert_eq!(out, [0, 0, 3]);
        assert_eq!(mnr.serial_to_choices(23, &mut out), true);
        assert_eq!(out, [1, 3, 4]);
        assert_eq!(mnr.serial_to_choices(34, &mut out), true);
        assert_eq!(out, [4, 4, 4]);
        assert_eq!(mnr.serial_to_choices(35, &mut out), false);
    }

    #[test]
    fn test_choices_to_serial() {
        let mnr = Mnr::<5, 3>::new();
        assert_eq!(mnr.choices_to_serial(&[0, 0, 0]), Some(0));
        assert_eq!(mnr.choices_to_serial(&[0, 0, 3]), Some(3));
        assert_eq!(mnr.choices_to_serial(&[1, 3, 4]), Some(23));
        assert_eq!(mnr.choices_to_serial(&[1, 3, 5]), None);
        assert_eq!(mnr.choices_to_serial(&[4, 4, 4]), Some(34));
    }
}
