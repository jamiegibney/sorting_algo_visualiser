use super::*;

mod lsd;
mod lsd_in_place;
mod msd;
mod msd_in_place;

pub(super) use lsd::RadixLSD;
pub(super) use lsd_in_place::RadixLSDInPlace;
pub(super) use msd::RadixMSD;
pub(super) use msd_in_place::RadixMSDInPlace;

pub(self) const fn get_digit(a: usize, power: usize, radix: usize) -> usize {
    let x = radix.pow(power as u32);
    (a / x) % radix
}

pub(self) fn analyze(
    arr: &mut SortArray,
    analysis: &mut Vec<usize>,
    base: usize,
) -> usize {
    analysis.resize(arr.len(), 0);
    let mut a = 0;

    let log_base = (base as f32).ln();

    for i in 0..arr.len() {
        analysis[i] = 1;
        let log_arr_i = (arr.read(i) as f32).ln();
        let x = (log_arr_i / log_base) as usize;

        if x > a {
            a = x;
        }
    }

    a
}
