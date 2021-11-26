// ---------------------------------------------------------------------------
// Copyright:   (c) 2021 ff. Michael Amrhein (michael@adrhinum.de)
// License:     This program is part of a larger application. For license
//              details please read the file LICENSE.TXT provided together
//              with the application.
// ---------------------------------------------------------------------------
// $Source$
// $Revision$

use std::{fmt::Debug, ops::Mul, str::FromStr};

use fpdec::{Decimal, DivRounded, MulRounded};
use rust_decimal;

pub static ORDER_DETAILS: [(u32, &str, &str); 300] = [
    (4, "19.95", "17.50"),
    (49, "49.90", "30.00"),
    (78, "7.95", "15.00"),
    (126, "7.95", "30.00"),
    (100, "12.95", "15.00"),
    (146, "7.95", "17.50"),
    (8, "12.95", "30.00"),
    (59, "19.95", "30.00"),
    (58, "49.90", "30.00"),
    (26, "7.95", "17.50"),
    (125, "19.95", "15.00"),
    (26, "49.90", "30.00"),
    (87, "49.90", "15.00"),
    (30, "12.95", "30.00"),
    (111, "19.95", "15.00"),
    (46, "48.00", "17.50"),
    (132, "49.90", "17.50"),
    (96, "32.00", "15.00"),
    (139, "32.00", "17.50"),
    (57, "19.95", "17.50"),
    (65, "12.95", "15.00"),
    (49, "32.00", "17.50"),
    (62, "48.00", "15.00"),
    (25, "48.00", "30.00"),
    (137, "32.00", "15.00"),
    (120, "7.95", "15.00"),
    (59, "32.00", "15.00"),
    (19, "7.95", "15.00"),
    (52, "7.95", "17.50"),
    (114, "49.90", "15.00"),
    (143, "32.00", "15.00"),
    (79, "48.00", "17.50"),
    (107, "48.00", "15.00"),
    (5, "48.00", "15.00"),
    (2, "7.95", "17.50"),
    (123, "48.00", "15.00"),
    (89, "7.95", "17.50"),
    (63, "32.00", "17.50"),
    (19, "7.95", "15.00"),
    (92, "12.95", "15.00"),
    (53, "48.00", "15.00"),
    (72, "12.95", "30.00"),
    (116, "48.00", "30.00"),
    (61, "48.00", "30.00"),
    (5, "7.95", "17.50"),
    (108, "19.95", "17.50"),
    (71, "7.95", "15.00"),
    (59, "32.00", "17.50"),
    (93, "32.00", "17.50"),
    (45, "7.95", "17.50"),
    (140, "48.00", "17.50"),
    (43, "32.00", "17.50"),
    (28, "48.00", "15.00"),
    (55, "19.95", "17.50"),
    (87, "48.00", "17.50"),
    (126, "49.90", "30.00"),
    (18, "19.95", "30.00"),
    (80, "49.90", "30.00"),
    (7, "7.95", "15.00"),
    (109, "19.95", "17.50"),
    (137, "49.90", "17.50"),
    (6, "7.95", "17.50"),
    (73, "19.95", "30.00"),
    (57, "32.00", "30.00"),
    (37, "49.90", "15.00"),
    (20, "12.95", "15.00"),
    (40, "19.95", "15.00"),
    (66, "32.00", "15.00"),
    (111, "49.90", "17.50"),
    (82, "49.90", "15.00"),
    (28, "12.95", "15.00"),
    (66, "48.00", "17.50"),
    (144, "48.00", "30.00"),
    (68, "48.00", "15.00"),
    (131, "32.00", "30.00"),
    (143, "12.95", "15.00"),
    (85, "49.90", "30.00"),
    (109, "32.00", "30.00"),
    (43, "12.95", "15.00"),
    (88, "19.95", "15.00"),
    (126, "12.95", "17.50"),
    (56, "19.95", "30.00"),
    (120, "32.00", "15.00"),
    (101, "12.95", "17.50"),
    (106, "48.00", "30.00"),
    (107, "12.95", "15.00"),
    (28, "19.95", "30.00"),
    (1, "7.95", "17.50"),
    (13, "12.95", "17.50"),
    (88, "49.90", "15.00"),
    (37, "49.90", "17.50"),
    (92, "49.90", "15.00"),
    (1, "12.95", "17.50"),
    (18, "12.95", "30.00"),
    (25, "19.95", "15.00"),
    (106, "32.00", "17.50"),
    (32, "7.95", "15.00"),
    (32, "49.90", "15.00"),
    (123, "48.00", "17.50"),
    (56, "49.90", "17.50"),
    (85, "7.95", "15.00"),
    (5, "7.95", "17.50"),
    (120, "12.95", "15.00"),
    (45, "12.95", "15.00"),
    (6, "32.00", "17.50"),
    (110, "19.95", "30.00"),
    (50, "7.95", "17.50"),
    (91, "7.95", "17.50"),
    (106, "12.95", "30.00"),
    (84, "32.00", "15.00"),
    (22, "12.95", "17.50"),
    (4, "19.95", "15.00"),
    (99, "49.90", "30.00"),
    (70, "19.95", "30.00"),
    (81, "48.00", "30.00"),
    (79, "12.95", "15.00"),
    (85, "32.00", "15.00"),
    (142, "19.95", "15.00"),
    (126, "7.95", "17.50"),
    (40, "49.90", "15.00"),
    (149, "32.00", "17.50"),
    (117, "32.00", "30.00"),
    (130, "48.00", "17.50"),
    (44, "12.95", "17.50"),
    (46, "7.95", "15.00"),
    (47, "49.90", "30.00"),
    (7, "49.90", "30.00"),
    (82, "7.95", "15.00"),
    (87, "48.00", "30.00"),
    (136, "7.95", "15.00"),
    (149, "32.00", "15.00"),
    (78, "19.95", "15.00"),
    (21, "48.00", "15.00"),
    (35, "19.95", "30.00"),
    (57, "19.95", "15.00"),
    (142, "49.90", "17.50"),
    (76, "12.95", "15.00"),
    (8, "12.95", "15.00"),
    (12, "19.95", "17.50"),
    (76, "48.00", "30.00"),
    (73, "12.95", "17.50"),
    (95, "48.00", "15.00"),
    (1, "19.95", "30.00"),
    (97, "49.90", "17.50"),
    (69, "48.00", "30.00"),
    (67, "49.90", "15.00"),
    (89, "7.95", "30.00"),
    (140, "49.90", "17.50"),
    (87, "12.95", "17.50"),
    (14, "12.95", "30.00"),
    (118, "19.95", "15.00"),
    (71, "19.95", "30.00"),
    (54, "49.90", "17.50"),
    (30, "32.00", "30.00"),
    (120, "32.00", "17.50"),
    (48, "49.90", "17.50"),
    (81, "48.00", "15.00"),
    (79, "49.90", "17.50"),
    (122, "12.95", "30.00"),
    (106, "49.90", "30.00"),
    (76, "7.95", "30.00"),
    (60, "32.00", "17.50"),
    (143, "12.95", "30.00"),
    (6, "48.00", "30.00"),
    (89, "7.95", "17.50"),
    (20, "7.95", "15.00"),
    (121, "48.00", "15.00"),
    (121, "19.95", "17.50"),
    (82, "19.95", "15.00"),
    (96, "12.95", "15.00"),
    (53, "48.00", "30.00"),
    (97, "48.00", "17.50"),
    (22, "49.90", "30.00"),
    (40, "7.95", "30.00"),
    (27, "49.90", "15.00"),
    (128, "32.00", "15.00"),
    (63, "7.95", "15.00"),
    (79, "7.95", "17.50"),
    (88, "49.90", "17.50"),
    (102, "49.90", "15.00"),
    (54, "48.00", "15.00"),
    (2, "48.00", "17.50"),
    (104, "32.00", "15.00"),
    (69, "19.95", "17.50"),
    (23, "19.95", "15.00"),
    (48, "32.00", "17.50"),
    (87, "32.00", "30.00"),
    (96, "7.95", "17.50"),
    (77, "7.95", "15.00"),
    (48, "49.90", "30.00"),
    (4, "7.95", "17.50"),
    (108, "12.95", "30.00"),
    (31, "19.95", "17.50"),
    (36, "32.00", "30.00"),
    (148, "12.95", "30.00"),
    (133, "48.00", "30.00"),
    (27, "7.95", "30.00"),
    (117, "32.00", "30.00"),
    (15, "32.00", "17.50"),
    (105, "7.95", "17.50"),
    (129, "12.95", "17.50"),
    (107, "48.00", "17.50"),
    (106, "48.00", "30.00"),
    (64, "32.00", "30.00"),
    (119, "49.90", "30.00"),
    (111, "7.95", "15.00"),
    (150, "19.95", "30.00"),
    (45, "7.95", "30.00"),
    (88, "48.00", "30.00"),
    (14, "49.90", "17.50"),
    (16, "48.00", "30.00"),
    (55, "7.95", "17.50"),
    (123, "48.00", "15.00"),
    (15, "32.00", "30.00"),
    (85, "48.00", "17.50"),
    (14, "7.95", "15.00"),
    (11, "19.95", "17.50"),
    (144, "19.95", "15.00"),
    (100, "12.95", "17.50"),
    (23, "12.95", "15.00"),
    (31, "48.00", "30.00"),
    (138, "48.00", "30.00"),
    (33, "48.00", "15.00"),
    (36, "19.95", "30.00"),
    (148, "12.95", "15.00"),
    (99, "32.00", "17.50"),
    (71, "32.00", "15.00"),
    (95, "19.95", "17.50"),
    (100, "7.95", "17.50"),
    (139, "12.95", "15.00"),
    (106, "7.95", "30.00"),
    (81, "7.95", "17.50"),
    (122, "49.90", "17.50"),
    (96, "7.95", "30.00"),
    (82, "7.95", "17.50"),
    (148, "49.90", "15.00"),
    (8, "48.00", "17.50"),
    (14, "48.00", "17.50"),
    (140, "48.00", "15.00"),
    (82, "19.95", "15.00"),
    (26, "12.95", "17.50"),
    (94, "32.00", "17.50"),
    (22, "19.95", "30.00"),
    (79, "32.00", "17.50"),
    (51, "48.00", "17.50"),
    (61, "49.90", "30.00"),
    (62, "49.90", "30.00"),
    (143, "49.90", "15.00"),
    (19, "7.95", "30.00"),
    (8, "32.00", "17.50"),
    (123, "32.00", "15.00"),
    (142, "7.95", "17.50"),
    (77, "19.95", "17.50"),
    (111, "32.00", "17.50"),
    (116, "48.00", "30.00"),
    (27, "19.95", "17.50"),
    (31, "12.95", "17.50"),
    (87, "12.95", "17.50"),
    (120, "49.90", "30.00"),
    (74, "7.95", "17.50"),
    (124, "19.95", "30.00"),
    (94, "7.95", "17.50"),
    (139, "32.00", "15.00"),
    (79, "12.95", "15.00"),
    (13, "12.95", "30.00"),
    (78, "19.95", "17.50"),
    (147, "19.95", "15.00"),
    (93, "7.95", "15.00"),
    (140, "32.00", "30.00"),
    (73, "48.00", "15.00"),
    (33, "48.00", "15.00"),
    (37, "19.95", "30.00"),
    (83, "48.00", "15.00"),
    (69, "32.00", "17.50"),
    (101, "32.00", "30.00"),
    (12, "49.90", "30.00"),
    (78, "7.95", "17.50"),
    (116, "32.00", "15.00"),
    (104, "12.95", "17.50"),
    (116, "7.95", "15.00"),
    (84, "7.95", "30.00"),
    (37, "48.00", "30.00"),
    (77, "19.95", "30.00"),
    (41, "7.95", "30.00"),
    (83, "12.95", "15.00"),
    (46, "48.00", "17.50"),
    (11, "48.00", "30.00"),
    (104, "12.95", "15.00"),
    (33, "19.95", "15.00"),
    (68, "48.00", "17.50"),
    (22, "7.95", "17.50"),
    (109, "48.00", "15.00"),
    (64, "12.95", "17.50"),
    (141, "7.95", "17.50"),
    (43, "49.90", "30.00"),
    (21, "19.95", "17.50"),
    (35, "32.00", "30.00"),
    (9, "49.90", "17.50"),
    (7, "19.95", "15.00"),
    (55, "32.00", "17.50"),
];

pub struct OrderDetail<T: Copy + FromStr> {
    quantity: u32,
    price: T,
    discount: T,
}

impl<T: Copy + FromStr> OrderDetail<T> {
    fn new(quantity: u32, price: T, discount: T) -> Self {
        OrderDetail {
            quantity,
            price,
            discount,
        }
    }
}

pub struct Order<T: Copy + FromStr> {
    pub details: Vec<OrderDetail<T>>,
}

pub trait OrderBuilder<T>
where
    T: Copy + FromStr,
    <T as FromStr>::Err: Debug,
{
    fn build_order(order_details: &[(u32, &str, &str)]) -> Order<T> {
        let mut order = Order::<T> { details: vec![] };
        for (qty, price_str, discount_str) in order_details {
            order.details.push(OrderDetail::new(
                *qty,
                T::from_str(*price_str).unwrap(),
                T::from_str(*discount_str).unwrap(),
            ));
        }
        order
    }
}

impl OrderBuilder<Decimal> for Order<Decimal> {}

impl OrderBuilder<f64> for Order<f64> {}

impl OrderBuilder<rust_decimal::Decimal> for Order<rust_decimal::Decimal> {}

#[inline(always)]
fn round2(val: f64) -> f64 {
    (val * 100_f64).round() / 100_f64
}

#[inline(always)]
fn round9(val: f64) -> f64 {
    (val * 1000000000_f64).round() / 1000000000_f64
}

pub trait OrderCalculator<T: Copy + FromStr> {
    fn calc_order(&self) -> (u32, T);
}

impl OrderCalculator<Decimal> for Order<Decimal> {
    fn calc_order(&self) -> (u32, Decimal) {
        let mut total_qty = 0_u32;
        let mut total_value = Decimal::ZERO;
        for dtl in &self.details {
            let rate: Decimal = (100_u8 - dtl.discount).div_rounded(100_u8, 9);
            let net_price: Decimal = dtl.price.mul_rounded(rate, 2);
            let dtl_total = dtl.quantity * net_price;
            total_qty += dtl.quantity;
            total_value = total_value + dtl_total;
        }
        (total_qty, total_value)
    }
}

impl OrderCalculator<f64> for Order<f64> {
    fn calc_order(&self) -> (u32, f64) {
        let mut total_qty = 0_u32;
        let mut total_value = 0_f64;
        for dtl in &self.details {
            let rate = round9((100_f64 - dtl.discount) / 100_f64);
            let net_price = round2(dtl.price.mul(rate));
            let dtl_total = dtl.quantity as f64 * net_price;
            total_qty += dtl.quantity;
            total_value = total_value + dtl_total;
        }
        (total_qty, total_value)
    }
}

impl OrderCalculator<rust_decimal::Decimal> for Order<rust_decimal::Decimal> {
    fn calc_order(&self) -> (u32, rust_decimal::Decimal) {
        let mut total_qty = 0_u32;
        let mut total_value = rust_decimal::Decimal::ZERO;
        for dtl in &self.details {
            let rate: rust_decimal::Decimal =
                ((rust_decimal::Decimal::ONE_HUNDRED - dtl.discount)
                    / rust_decimal::Decimal::ONE_HUNDRED)
                    .round_dp(9);
            let net_price = dtl.price.mul(rate).round_dp(2);
            let dtl_total =
                rust_decimal::Decimal::from(dtl.quantity) * net_price;
            total_qty += dtl.quantity;
            total_value = total_value + dtl_total;
        }
        (total_qty, total_value)
    }
}

#[cfg(test)]
mod test_order_with_fpdec {
    use fpdec::Dec;

    use crate::*;

    #[test]
    fn test_order() {
        let order = Order::<Decimal>::build_order(&ORDER_DETAILS);
        let (total_qty, total_value) = order.calc_order();
        assert_eq!(total_qty, 22040);
        assert_eq!(total_value, Dec!(507505.86));
    }
}

#[cfg(test)]
mod test_order_with_f64 {
    use crate::*;

    #[test]
    fn test_order() {
        let order = Order::<f64>::build_order(&ORDER_DETAILS);
        let (total_qty, total_value) = order.calc_order();
        assert_eq!(total_qty, 22040);
        assert_eq!(total_value.round(), 507521.);
    }
}

#[cfg(test)]
mod test_order_with_rust_decimal {
    use rust_decimal_macros::dec;

    use crate::*;

    #[test]
    fn test_order() {
        let order = Order::<rust_decimal::Decimal>::build_order(&ORDER_DETAILS);
        let (total_qty, total_value) = order.calc_order();
        assert_eq!(total_qty, 22040);
        assert_eq!(total_value, dec!(507505.86));
    }
}