#![allow(unused)]
use dirs;
use std::path::PathBuf;
pub mod state;

pub fn sqrt(y: u64) -> u64 {
    if (y < 4) {
        if (y == 0) {
            0u64
        } else {
            1u64
        }
    } else {
        let mut z = y;
        let mut x = y / 2 + 1;
        while (x < z) {
            z = x;
            x = (y / x + x) / 2;
        }
        z
    }
}

pub fn sui_sqrt(x: u64) -> u64 {
    let mut bit = 1u128 << 64;
    let mut res = 0u128;
    let mut x = (x as u128);

    while (bit != 0) {
        if (x >= res + bit) {
            x = x - (res + bit);
            res = (res >> 1) + bit;
        } else {
            res = res >> 1;
        };
        bit = bit >> 2;
    }

    (res as u64)
}

pub fn min(a: u64, b: u64) -> u64 {
    if a > b {
        b
    } else {
        a
    }
}

pub fn max(a: u64, b: u64) -> u64 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn quote(reserve_a: u64, reserve_b: u64, input_a: u64) -> u64 {
    (reserve_b / reserve_a) * input_a
}

pub fn get_input(dx: u64, x: u64, y: u64, f: u64) -> u64 {
    let dx_fee_deduction = (10000 - f) * dx;
    let numerator = dx_fee_deduction * y;
    let denominator = 10000 * x + dx_fee_deduction;

    (numerator / denominator)
}

pub fn minted_lp_after_increase_liquidity(x: u64, y: u64, dx: u64, dy: u64, lp_supply: u64) -> u64 {
    min(dx * lp_supply / x, dy * lp_supply / y)
}

pub fn withdraw_liquidity(sui_r: u64, token_y_r: u64, lp_value: u64, lp_supply: u64) -> (u64, u64) {
    (
        (sui_r * lp_value / lp_supply),
        (token_y_r * lp_value / lp_supply),
    )
}

pub fn default_keystore_path() -> PathBuf {
    match dirs::home_dir() {
        ///$HOME/.sui/sui_config/sui.keystore
        Some(v) => v.join(".sui").join("sui_config").join("sui.keystore"),
        None => panic!("Cannot obtain home directory path"),
    }
}
