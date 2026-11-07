#![cfg(any(feature = "pyxel-core-backend", feature = "wasm-backend"))]
#[cfg(feature = "pyxel-core-backend")]
use pyxel::Pyxel as Official;
#[cfg(feature = "wasm-backend")]
use pyxel_cube_core::Pyxel as Official;
use pyxel_rust::api::*;

#[test]
fn seeded_random_matches_official_including_extreme_and_reversed_bounds() {
    for seed in [0, 1, 12345, u32::MAX] {
        Official::random_seed(seed);
        let expected: Vec<_> = (0..20)
            .map(|_| Official::random_int(i32::MIN, i32::MAX))
            .collect();
        rseed(seed);
        assert_eq!(
            (0..20)
                .map(|_| rndi(i32::MIN, i32::MAX))
                .collect::<Vec<_>>(),
            expected
        );
        Official::random_seed(seed);
        let expected: Vec<_> = (0..20).map(|_| Official::random_float(5., -3.)).collect();
        rseed(seed);
        assert_eq!((0..20).map(|_| rndf(5., -3.)).collect::<Vec<_>>(), expected);
    }
    rseed(42);
    let values: std::collections::HashSet<_> = (0..100).map(|_| rnd_int(10, None)).collect();
    assert!(
        values.len() > 5,
        "legacy integer RNG must not always return zero"
    );
    assert!(values.iter().all(|v| (0..10).contains(v)));
}
#[test]
fn noise_and_degree_math_match_official() {
    nseed(123);
    let values: Vec<_> = [(0.5, 0., 0.), (1.25, -3.5, 4.75)]
        .iter()
        .map(|&(x, y, z)| noise(x, Some(y), Some(z)))
        .collect();
    Official::noise_seed(123);
    assert_eq!(
        values,
        vec![
            Official::noise(0.5, 0., 0.),
            Official::noise(1.25, -3.5, 4.75)
        ]
    );
    nseed(456);
    assert_ne!(noise(1.25, Some(-3.5), Some(4.75)), values[1]);
    assert_eq!(noise(0.5, None, None), noise(0.5, Some(0.), Some(0.)));
    assert_eq!(ceil(-1.2), -1);
    assert_eq!(ceil(1.2), 2);
    assert_eq!(atan2(1., 0.), 90.);
    assert_eq!(atan2(0., -1.), Official::atan2(0., -1.));
}
