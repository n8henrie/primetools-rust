use primetools::*;

#[test]
fn test_primegen() {
    assert_eq!(
        PrimeGen::new(Some(23)).collect::<Vec<_>>(),
        [2, 3, 5, 7, 11, 13, 17, 19],
    );
}
#[test]
fn test_primesieve() {
    assert_eq!(PrimeSieve::new(23).0, [2, 3, 5, 7, 11, 13, 17, 19],);
}
