use std::mem::{forget, replace, MaybeUninit};

pub struct IntoArray<T: Sized, const U: usize> {
    pub array: [T; U],
}

impl<T, const U: usize> FromIterator<T> for IntoArray<T, U> {
    fn from_iter<V: IntoIterator<Item = T>>(iter: V) -> Self {
        let mut result: MaybeUninit<[T; U]> = MaybeUninit::uninit();

        for (i, value) in iter.into_iter().enumerate() {
            let array = result.as_mut_ptr();
            unsafe {
                let uninit_value = replace(&mut (*array)[i], value);
                forget(uninit_value);
            }
        }

        IntoArray {
            array: unsafe { result.assume_init() },
        }
    }
}
