use num_integer::Roots;
use crate::math::scalar::{
    rand::RandomDistributionBuilder,
    usf::{UsfScalar, UsfScalarConstants},
};

pub fn random_value_batch(loop_depth: u32) -> Vec<UsfScalar> {
    const TARGET_INNER_ITERATIONS: u32 = 100000;

    let count = match loop_depth {
        0 => TARGET_INNER_ITERATIONS,
        1..=10 => TARGET_INNER_ITERATIONS.nth_root(loop_depth),
        _ => panic!("Don't you think TEN nested loops are enough?!"),
    };
    let pos_one_decillion = <UsfScalar as UsfScalarConstants>::POSITIVE_ONE_DECILLION;
    let neg_one_decillion = <UsfScalar as UsfScalarConstants>::NEGATIVE_ONE_DECILLION;

    RandomDistributionBuilder::new()
        .component(1, |c| c.positive().less_than(pos_one_decillion))
        .component(1, |c| c.negative().greater_than(neg_one_decillion))
        .build(0xADDD_0001, count as usize)
}