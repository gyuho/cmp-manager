pub fn eq_vectors<T>(va: &[T], vb: &[T]) -> bool
where
    T: Eq,
{
    (va.len() == vb.len()) && va.iter().zip(vb).all(|(a, b)| *a == *b)
}

pub fn is_sorted_and_unique<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] < w[1])
}

/// RUST_LOG=debug cargo test --all-features --package cmp-manager --lib -- test_cmp --exact --show-output
#[test]
fn test_cmp() {
    assert!(eq_vectors(&[0], &[0]));
    assert!(!eq_vectors(&[0], &[1]));
    assert!(is_sorted_and_unique(&[0, 1]));
    assert!(is_sorted_and_unique(&[0, 1, 2]));
    assert!(!is_sorted_and_unique(&[1, 0]));
    assert!(!is_sorted_and_unique(&[0, 0]));
    assert!(!is_sorted_and_unique(&[0, 1, 1]));
}
