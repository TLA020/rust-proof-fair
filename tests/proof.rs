#[cfg(test)]
mod tests {
    use proof_fair::Proof;

    #[test]
    fn test_proof_new() {
        let proof = Proof::new(None, None, 0);
        assert_eq!(proof.nonce, 0);
        assert_eq!(proof.client_seed.len(), 64);
        assert_eq!(proof.server_seed.len(), 64);
        assert_eq!(proof.blinded_server_seed.len(), 32);
    }

    #[test]
    fn test_proof_roll() {
        let mut proof = Proof::new(None, None, 0);
        let result = proof.roll();
        assert!(result.is_ok());
        assert_eq!(proof.nonce, 1);
    }

    #[test]
    fn test_proof_calculate() {
        let proof = Proof::new(None, None, 0);
        let result = proof.calculate();
        assert!(result.is_ok());
    }
}
