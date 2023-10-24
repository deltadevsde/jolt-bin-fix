use ark_ff::PrimeField;

use super::JoltInstruction;
use crate::{
  jolt::subtable::{
    eq::EQSubtable, eq_abs::EQABSSubtable, eq_msb::EQMSBSubtable, gt_msb::GTMSBSubtable,
    lt_abs::LTABSSubtable, ltu::LTUSubtable, LassoSubtable,
  },
  utils::instruction_utils::chunk_and_concatenate_operands,
};

#[derive(Copy, Clone, Default, Debug)]
pub struct SLTInstruction(pub i64, pub i64);

impl JoltInstruction for SLTInstruction {
  fn combine_lookups<F: PrimeField>(&self, vals: &[F], C: usize, _: usize) -> F {
    debug_assert!(vals.len() % C == 0);
    let mut vals_by_subtable = vals.chunks_exact(C);

    let gt_msb = vals_by_subtable.next().unwrap();
    let eq_msb = vals_by_subtable.next().unwrap();
    let ltu = vals_by_subtable.next().unwrap();
    let eq = vals_by_subtable.next().unwrap();
    let lt_abs = vals_by_subtable.next().unwrap();
    let eq_abs = vals_by_subtable.next().unwrap();

    // Accumulator for LTU(x_{<s}, y_{<s})
    let mut ltu_sum = lt_abs[0];
    // Accumulator for EQ(x_{<s}, y_{<s})
    let mut eq_prod = eq_abs[0];
    for i in 1..C {
      ltu_sum += ltu[i] * eq_prod;
      eq_prod *= eq[i];
    }

    // x_s * (1 - y_s) + EQ(x_s, y_s) * LTU(x_{<s}, y_{<s})
    gt_msb[0] + eq_msb[0] * ltu_sum
  }

  fn g_poly_degree(&self, C: usize) -> usize {
    C
  }

  fn subtables<F: PrimeField>(&self) -> Vec<Box<dyn LassoSubtable<F>>> {
    vec![
      Box::new(GTMSBSubtable::new()),
      Box::new(EQMSBSubtable::new()),
      Box::new(LTUSubtable::new()),
      Box::new(EQSubtable::new()),
      Box::new(LTABSSubtable::new()),
      Box::new(EQABSSubtable::new()),
    ]
  }

  fn to_indices(&self, C: usize, log_M: usize) -> Vec<usize> {
    chunk_and_concatenate_operands(self.0 as u64, self.1 as u64, C, log_M)
  }
}

#[cfg(test)]
mod test {
  use ark_curve25519::Fr;
  use ark_std::{test_rng, One, Zero};
  use rand_chacha::rand_core::RngCore;

  use crate::{jolt::instruction::JoltInstruction, jolt_instruction_test};

  use super::SLTInstruction;

  #[test]
  fn slt_instruction_e2e() {
    let mut rng = test_rng();
    const C: usize = 8;
    const M: usize = 1 << 16;

    for _ in 0..256 {
      let x = rng.next_u64() as i64;
      let y = rng.next_u64() as i64;
      jolt_instruction_test!(SLTInstruction(x, y), (x < y).into());
    }
    for _ in 0..256 {
      let x = rng.next_u64() as i64;
      jolt_instruction_test!(SLTInstruction(x, x), Fr::zero());
    }
  }
}
