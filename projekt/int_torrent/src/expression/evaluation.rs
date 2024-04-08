use super::models::{AExpr, BinaryOperation};
impl AExpr {
    pub fn evaluate(&self) -> i64 {
        // Tukaj lahko predpostavite, da spremenljivke ne obstajajo
        panic!("Implement variable evaluation")
    }

    pub fn evaluate_map(
        &self,
        vars: &std::collections::HashMap<String, Option<i64>>,
    ) -> Option<i64> {
        panic!("Implement variable evaluation")
    }
}
