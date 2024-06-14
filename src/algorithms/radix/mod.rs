use super::*;

mod lsd;
mod lsd_in_place;
mod msd;

pub(super) use lsd::RadixLSD;
pub(super) use lsd_in_place::RadixLSDInPlace;
pub(super) use msd::RadixMSD;

fn get_digit(a: usize, power: usize, radix: usize) -> usize {
    let x = (radix as f32).powi(power as i32);
    (a as f32 / x) as usize % radix
}

fn max_power(arr: &mut SortArray, base: usize) -> usize {
    let mut a = 0;
    let log_base = (base as f32).ln();
    for i in 0..arr.len() {
        let log_arr_i = (arr.read(i) as f32).ln();
        let x = (log_arr_i / log_base) as usize;

        if x > a {
            a = x;
        }
    }

    a
}


