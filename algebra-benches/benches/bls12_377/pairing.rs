mod pairing {
    use algebra::UniformRand;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;

    use algebra::{
        bls12::{G1Prepared, G2Prepared},
        bls12_377::{
            Bls12_377, Fq12, G1Affine, G1Projective as G1, G2Affine, G2Projective as G2, Parameters,
        },
        PairingEngine,
    };

    #[bench]
    fn bench_pairing_miller_loop(b: &mut ::test::Bencher) {
        const SAMPLES: usize = 1000;

        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        let v: Vec<(G1Prepared<Parameters>, G2Prepared<Parameters>)> = (0..SAMPLES)
            .map(|_| {
                (
                    G1Affine::from(G1::rand(&mut rng)).into(),
                    G2Affine::from(G2::rand(&mut rng)).into(),
                )
            })
            .collect();

        let mut count = 0;
        b.iter(|| {
            let tmp = Bls12_377::miller_loop(&[(v[count].0.clone(), v[count].1.clone())]);
            count = (count + 1) % SAMPLES;
            tmp
        });
    }

    #[bench]
    fn bench_pairing_final_exponentiation(b: &mut ::test::Bencher) {
        const SAMPLES: usize = 1000;

        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        let v: Vec<Fq12> = (0..SAMPLES)
            .map(|_| {
                let p = G1Affine::from(G1::rand(&mut rng)).into();
                let q = G2Affine::from(G2::rand(&mut rng)).into();
                Bls12_377::miller_loop(&[(p, q)])
            })
            .collect();

        let mut count = 0;
        b.iter(|| {
            let tmp = Bls12_377::final_exponentiation(&v[count]);
            count = (count + 1) % SAMPLES;
            tmp
        });
    }

    #[bench]
    fn bench_pairing_full(b: &mut ::test::Bencher) {
        const SAMPLES: usize = 1000;

        let mut rng = XorShiftRng::seed_from_u64(1231275789u64);

        let v: Vec<(G1, G2)> = (0..SAMPLES)
            .map(|_| (G1::rand(&mut rng), G2::rand(&mut rng)))
            .collect();

        let mut count = 0;
        b.iter(|| {
            let tmp = Bls12_377::pairing(v[count].0, v[count].1);
            count = (count + 1) % SAMPLES;
            tmp
        });
    }
}
