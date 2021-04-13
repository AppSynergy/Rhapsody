use rand;
use rand::prelude::*;
use std::cmp;

use super::tiles::Element;

pub fn dx(x: u8, dice: u8) -> u8 {
    let max = cmp::min(dice, x);
    rand_u8(1, max)
}

pub fn d20() -> u8 {
    rand_u8(1, 20)
}

pub fn rand_u8(x: u8, y: u8) -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(x, y + 1) as u8
}

pub fn rand_element() -> Element {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, 4);
    [Element::Air, Element::Earth, Element::Fire, Element::Water][index]
}

pub fn binom(n: u8, p: f64) -> u8 {
    let mut rng = rand::thread_rng();
    let bin = rand::distributions::Binomial::new(n.into(), p);
    rng.sample(bin) as u8
}
