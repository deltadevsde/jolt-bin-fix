use crate::field::JoltField;
use ark_std::log2;
use std::marker::PhantomData;

use super::LassoSubtable;
use crate::utils::split_bits;

#[derive(Default)]
pub struct EqMSBSubtable<F: JoltField> {
    _field: PhantomData<F>,
}

impl<F: JoltField> EqMSBSubtable<F> {
    pub fn new() -> Self {
        Self {
            _field: PhantomData,
        }
    }
}

impl<F: JoltField> LassoSubtable<F> for EqMSBSubtable<F> {
    fn materialize(&self, M: usize) -> Vec<F> {
        let mut entries: Vec<F> = Vec::with_capacity(M);
        let bits_per_operand = (log2(M) / 2) as usize;
        let high_bit = 1usize << (bits_per_operand - 1);

        // Materialize table entries in order from 0..M
        for idx in 0..M {
            let (x, y) = split_bits(idx, bits_per_operand);
            let row = (x & high_bit) == (y & high_bit);
            entries.push(if row { F::one() } else { F::zero() });
        }
        entries
    }

    fn evaluate_mle(&self, point: &[F]) -> F {
        debug_assert!(point.len() % 2 == 0);
        let b = point.len() / 2;
        let (x, y) = point.split_at(b);
        // x_0 * y_0 + (1 - x_0) * (1 - y_0)
        x[0] * y[0] + (F::one() - x[0]) * (F::one() - y[0])
    }
}

#[cfg(test)]
mod test {
    use ark_bn254::Fr;
    use binius_field::BinaryField128b;

    use crate::{
        field::binius::BiniusField,
        jolt::subtable::{eq_msb::EqMSBSubtable, LassoSubtable},
        subtable_materialize_mle_parity_test,
    };

    subtable_materialize_mle_parity_test!(
        eq_msb_materialize_mle_parity,
        EqMSBSubtable<Fr>,
        Fr,
        256
    );
    subtable_materialize_mle_parity_test!(
        eq_msb_binius_materialize_mle_parity,
        EqMSBSubtable<BiniusField<BinaryField128b>>,
        BiniusField<BinaryField128b>,
        1 << 16
    );
}
