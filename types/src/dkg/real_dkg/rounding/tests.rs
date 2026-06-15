// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::dkg::real_dkg::rounding::{
    is_valid_profile, total_weight_lower_bound, total_weight_upper_bound, DKGRounding,
    DKGRoundingProfile, DEFAULT_FAST_PATH_SECRECY_THRESHOLD, DEFAULT_RECONSTRUCT_THRESHOLD,
    DEFAULT_SECRECY_THRESHOLD,
};
use aptos_dkg::pvss::WeightedConfig;
use claims::assert_le;
use fixed::types::U64F64;
use rand::{thread_rng, Rng};
use std::ops::Deref;

#[test]
fn compute_mainnet_rounding() {
    let validator_stakes = MAINNET_STAKES.to_vec();
    let dkg_rounding = DKGRounding::new(
        &validator_stakes,
        *DEFAULT_SECRECY_THRESHOLD.deref(),
        *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
        Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD.deref()),
    );
    println!("mainnet rounding profile: {:?}", dkg_rounding.profile);
    // Result:
    // mainnet rounding profile: total_weight: 414, secrecy_threshold_in_stake_ratio: 0.5, reconstruct_threshold_in_stake_ratio: 0.60478401144595166257, reconstruct_threshold_in_weights: 228, fast_reconstruct_threshold_in_stake_ratio: Some(0.7714506781126183292), fast_reconstruct_threshold_in_weights: Some(335), validator_weights: [7, 5, 6, 6, 5, 1, 6, 6, 1, 5, 6, 5, 1, 7, 1, 6, 6, 1, 2, 1, 6, 3, 2, 1, 1, 4, 3, 2, 5, 5, 5, 1, 1, 4, 1, 1, 1, 7, 5, 1, 1, 2, 6, 1, 6, 1, 3, 5, 5, 1, 5, 5, 3, 2, 5, 1, 6, 3, 6, 1, 1, 3, 1, 5, 1, 9, 1, 1, 1, 6, 1, 5, 7, 4, 6, 1, 5, 6, 5, 5, 3, 1, 6, 7, 6, 1, 3, 1, 1, 1, 1, 1, 1, 7, 2, 1, 6, 7, 1, 1, 1, 1, 5, 3, 1, 2, 3, 1, 1, 1, 1, 4, 1, 1, 1, 2, 1, 6, 7, 5, 1, 5, 1, 6, 1, 2, 3, 2, 2]

    let total_weight_min = total_weight_lower_bound(&validator_stakes);
    let total_weight_max = total_weight_upper_bound(
        &validator_stakes,
        *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
        *DEFAULT_SECRECY_THRESHOLD.deref(),
    );
    let total_weight = dkg_rounding.profile.validator_weights.iter().sum::<u64>();
    assert!(total_weight >= total_weight_min as u64);
    assert!(total_weight <= total_weight_max as u64);

    assert!(is_valid_profile(
        &dkg_rounding.profile,
        *DEFAULT_RECONSTRUCT_THRESHOLD.deref()
    ));
}

#[test]
fn test_rounding_single_validator() {
    let validator_stakes = vec![1_000_000];
    let dkg_rounding = DKGRounding::new(
        &validator_stakes,
        *DEFAULT_SECRECY_THRESHOLD.deref(),
        *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
        Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD.deref()),
    );
    let wconfig = WeightedConfig::new(1, vec![1]).unwrap();
    assert_eq!(dkg_rounding.wconfig, wconfig);
}

#[test]
fn test_rounding_equal_stakes() {
    let num_runs = 100;
    let mut rng = rand::thread_rng();
    for _ in 0..num_runs {
        let validator_num = rng.gen_range(100, 500);
        let validator_stakes = vec![1_000_000; validator_num];
        let dkg_rounding = DKGRounding::new(
            &validator_stakes,
            *DEFAULT_SECRECY_THRESHOLD.deref(),
            *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
            Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD.deref()),
        );
        let wconfig = WeightedConfig::new(
            (U64F64::from_num(validator_num) * *DEFAULT_SECRECY_THRESHOLD.deref())
                .ceil()
                .to_num::<usize>()
                + 1,
            vec![1; validator_num],
        )
        .unwrap();
        assert_eq!(dkg_rounding.wconfig, wconfig);
    }
}

#[test]
fn test_rounding_small_stakes() {
    let num_runs = 100;
    let mut rng = rand::thread_rng();
    for _ in 0..num_runs {
        let validator_num = rng.gen_range(1, 500);
        let mut validator_stakes = vec![];
        for _ in 0..validator_num {
            validator_stakes.push(rng.gen_range(1, 10));
        }
        let dkg_rounding = DKGRounding::new(
            &validator_stakes,
            *DEFAULT_SECRECY_THRESHOLD.deref(),
            *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
            Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD.deref()),
        );

        let total_weight_min = total_weight_lower_bound(&validator_stakes);
        let total_weight_max = total_weight_upper_bound(
            &validator_stakes,
            *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
            *DEFAULT_SECRECY_THRESHOLD.deref(),
        );
        let total_weight = dkg_rounding.profile.validator_weights.iter().sum::<u64>();
        assert!(total_weight >= total_weight_min as u64);
        assert!(total_weight <= total_weight_max as u64);
        assert!(is_valid_profile(
            &dkg_rounding.profile,
            *DEFAULT_RECONSTRUCT_THRESHOLD.deref()
        ));
    }
}

#[test]
fn test_rounding_uniform_distribution() {
    let num_runs = 100;
    let mut rng = rand::thread_rng();
    // assuming each validator has a stake between 1_000_000 and 50_000_000, following uniform distribution
    // randomly generate 100~500 validators' stake distribution
    for _ in 0..num_runs {
        let validator_num = rng.gen_range(100, 500);
        let mut validator_stakes = vec![];
        for _ in 0..validator_num {
            validator_stakes.push(rng.gen_range(1_000_000, 50_000_000));
        }
        let dkg_rounding = DKGRounding::new(
            &validator_stakes,
            *DEFAULT_SECRECY_THRESHOLD.deref(),
            *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
            Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD.deref()),
        );

        let total_weight_min = total_weight_lower_bound(&validator_stakes);
        let total_weight_max = total_weight_upper_bound(
            &validator_stakes,
            *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
            *DEFAULT_SECRECY_THRESHOLD.deref(),
        );
        let total_weight = dkg_rounding.profile.validator_weights.iter().sum::<u64>();
        assert!(total_weight >= total_weight_min as u64);
        assert!(total_weight <= total_weight_max as u64);
        assert!(is_valid_profile(
            &dkg_rounding.profile,
            *DEFAULT_RECONSTRUCT_THRESHOLD.deref()
        ));
    }
}

#[cfg(test)]
pub fn generate_approximate_zipf(size: usize, a: u64, b: u64, exponent: f64) -> Vec<u64> {
    use num_traits::Float;

    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| {
            let random_uniform = rng.gen_range(0.0, 1.0);
            let approximate_value =
                a + ((b - a + 1) as f64 * (1.0 - random_uniform).powf(exponent)) as u64;
            // Adjust value to be within the specified range [a, b]
            approximate_value.clamp(a, b)
        })
        .collect()
}

#[test]
fn test_rounding_zipf_distribution() {
    let num_runs = 100;
    let mut rng = rand::thread_rng();
    // assuming each validator has a stake between 1_000_000 and 50_000_000, following zipf distribution
    // randomly generate 100~500 validators' stake distribution
    for _ in 0..num_runs {
        let validator_num = rng.gen_range(100, 500);
        let validator_stakes = generate_approximate_zipf(validator_num, 1_000_000, 50_000_000, 5.0);
        let dkg_rounding = DKGRounding::new(
            &validator_stakes,
            *DEFAULT_SECRECY_THRESHOLD.deref(),
            *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
            Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD.deref()),
        );

        let total_weight_min = total_weight_lower_bound(&validator_stakes);
        let total_weight_max = total_weight_upper_bound(
            &validator_stakes,
            *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
            *DEFAULT_SECRECY_THRESHOLD.deref(),
        );
        let total_weight = dkg_rounding.profile.validator_weights.iter().sum::<u64>();
        assert!(total_weight >= total_weight_min as u64);
        assert!(total_weight <= total_weight_max as u64);
        assert!(is_valid_profile(
            &dkg_rounding.profile,
            *DEFAULT_RECONSTRUCT_THRESHOLD.deref()
        ));
    }
}

#[cfg(test)]
pub const MAINNET_STAKES: [u64; 129] = [
    145363920367444000,
    100779935896493000,
    134154255721034000,
    134234783226671000,
    103686549105772000,
    23681356495150300,
    134577645972875000,
    134580712197205000,
    10857449995312600,
    105977831715137000,
    120872333189108000,
    102057954967375000,
    25968006480319300,
    150210261808047000,
    11047428320304400,
    134184685206018000,
    128117795425337000,
    12497398680912800,
    50533200268704000,
    10856898438192700,
    134176595090811000,
    60011869592362800,
    39694335719301200,
    12863458468719700,
    11046541966772300,
    94427102742955200,
    58714132394437700,
    40145680094791300,
    100137028609146000,
    111809600151787000,
    114998912365121000,
    17951622559336400,
    10857216258421800,
    94429160256130900,
    11047450111666100,
    10856891072965200,
    10857020952663600,
    162161975531451000,
    104073248041097000,
    10857355385008300,
    10901683472171900,
    50198365896562800,
    134182311143049000,
    10857000567453500,
    134562155405120000,
    11046814332439800,
    60322390260321900,
    101055534219498000,
    114872371828424000,
    10929001624435000,
    106067388838015000,
    100152796096971000,
    54964069016926100,
    34040647636753800,
    102049697282067000,
    10856874306102900,
    134514110645862000,
    70499650588096400,
    134224065841861000,
    10857370648417500,
    28409091651485400,
    64302033587393400,
    16659350234613100,
    112568696851409000,
    10857782940276500,
    200882335168720000,
    10856846459964200,
    10856305691600600,
    10856576121655500,
    123961368695808000,
    20275491671732600,
    112069796227716000,
    148078356637657000,
    76893226659146600,
    135298123702389000,
    10856788596777500,
    107821720522194000,
    134626203055928000,
    106466193065101000,
    102103040930732000,
    62682920098289700,
    26223235705449200,
    134234424849999000,
    150210282994581000,
    134913703983987000,
    10857227273097400,
    57413947132891200,
    10900450777364100,
    12022049676664200,
    11047053887431400,
    10857590490261700,
    10857257627847700,
    26223774458854400,
    145363467800313000,
    49332020110088600,
    11047476999099800,
    126201751573407000,
    150458532010203000,
    10856470759531000,
    10857203409232300,
    11047327948099800,
    10856521489540500,
    99511242999587700,
    74202213386306600,
    11047051193450900,
    32601393370365500,
    70855459554627300,
    10857401127909800,
    25271130862928600,
    18684565615586300,
    10876016328079300,
    95866473180260000,
    10857461985291600,
    10857176446687100,
    17291414467856800,
    47528452645556600,
    17051109168257700,
    134912746458206000,
    150461059796590000,
    116315225160302000,
    10855453497812600,
    100865713263752000,
    10928177475521100,
    124293961561247000,
    26223786196810300,
    39221522777191400,
    73810826128117800,
    53685896908423700,
    40216803848486900,
];

#[test]
fn test_infallible_rounding_with_mainnet() {
    let profile = DKGRoundingProfile::infallible(
        &MAINNET_STAKES.to_vec(),
        *DEFAULT_SECRECY_THRESHOLD,
        *DEFAULT_RECONSTRUCT_THRESHOLD,
        Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD),
    );
    println!("profile={:?}", profile);
}

#[test]
fn test_infallible_rounding_brute_force() {
    let mut rng = thread_rng();
    let two = U64F64::from_num(2);
    for n in 1..=20 {
        let n_fixed = U64F64::from_num(n);
        let n_halved = n_fixed / 2;
        for _ in 0..10 {
            let stakes: Vec<u64> = (0..n).map(|_| rng.gen_range(1, 100)).collect();
            let stake_total = U64F64::from_num(stakes.clone().into_iter().sum::<u64>());
            let stake_secrecy_threshold = stake_total * *DEFAULT_SECRECY_THRESHOLD;
            let stake_reconstruct_threshold = stake_total * *DEFAULT_RECONSTRUCT_THRESHOLD;
            let fast_path_stake_secrecy_threshold =
                stake_total * *DEFAULT_FAST_PATH_SECRECY_THRESHOLD;
            let profile = DKGRoundingProfile::infallible(
                &stakes,
                *DEFAULT_SECRECY_THRESHOLD,
                *DEFAULT_RECONSTRUCT_THRESHOLD,
                Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD),
            );
            println!("n={}, stakes={:?}, profile={:?}", n, stakes, profile);
            let num_subsets: u64 = 1 << n;
            let weight_total = U64F64::from_num(profile.validator_weights.iter().sum::<u64>());

            // With default thresholds, weight_total <= (n/2 + 2)/(recon_threshold - secrecy_threshold) + rounding_weight_gain_total <= ceil((n/2 + 2)/(recon_threshold - secrecy_threshold)) + n/2
            assert_le!(
                weight_total,
                ((n_halved + two) / (*DEFAULT_RECONSTRUCT_THRESHOLD - *DEFAULT_SECRECY_THRESHOLD))
                    .ceil()
                    + n_halved
            );

            for subset in 0..num_subsets {
                let stake_sub_total = U64F64::from(get_sub_total(stakes.as_slice(), subset));
                let weight_sub_total = get_sub_total(profile.validator_weights.as_slice(), subset);
                if stake_sub_total <= stake_secrecy_threshold
                    && weight_sub_total >= profile.reconstruct_threshold_in_weights
                {
                    unreachable!();
                }
                if stake_sub_total > stake_reconstruct_threshold
                    && weight_sub_total < profile.reconstruct_threshold_in_weights
                {
                    unreachable!();
                }
                if stake_sub_total <= fast_path_stake_secrecy_threshold
                    && weight_sub_total >= profile.fast_reconstruct_threshold_in_weights.unwrap()
                {
                    unreachable!();
                }
            }
        }
    }
}

fn get_sub_total(vals: &[u64], subset: u64) -> u64 {
    vals.iter()
        .enumerate()
        .map(|(idx, &val)| val * ((subset >> idx) & 1))
        .sum()
}

/// Reproduces finding `genesis-subether-voting-power-zero-dkg-panic` (severity HIGH).
///
/// Genesis voting power is specified in wei and truncated to the u64 consensus voting
/// power by integer-dividing by 10^18 (`gravity_cli` waypoint.rs and the runtime greth
/// `wei_to_ether`). Any genesis validator staked with < 1 ether therefore gets consensus
/// `voting_power == 0`, and nothing on the active genesis path rejects this (the
/// genesis-tool only asserts `stakeAmount == votingPower` on the raw wei values, and the
/// Solidity `_initializeGenesisValidator` bypasses the `minimumBond` / `votingPower > 0`
/// check). The truncated voting power is fed directly as the DKG stake
/// (`real_dkg/mod.rs`: `validator_stakes = next_validators.map(|vi| vi.voting_power)`).
///
/// If the whole genesis set is sub-ether, every `voting_power == 0`, so `stake_sum == 0`
/// inside `compute_profile_fixed_point`. Line 332,
///   `stake_gap_fixed = stake_per_weight * delta_total_fixed / stake_sum_fixed`
/// divides by `stake_sum_fixed == 0` -> the U64F64 fixed-point arithmetic panics with
/// "division by zero". This happens in the very first `compute_profile_fixed_point`
/// call inside `DKGRoundingProfile::new` (the `best_profile` computation at
/// `rounding/mod.rs:204`, BEFORE `is_valid_profile` and before the `Err -> infallible`
/// fallback arm), and `infallible` itself would divide by zero the same way. So there is
/// no fallback: `DKGRounding::new` aborts the thread.
///
/// Because the DKG inputs (the on-chain validator set) are byte-identical across all
/// honest nodes, every node panics at the same epoch boundary: deterministic,
/// simultaneous crash = chain halt with no recovery path (re-genesis required).
///
/// This test demonstrates the defect by asserting `DKGRounding::new` panics for an
/// all-zero validator set. A correct implementation would instead return an error (or
/// reject the set at genesis), so this `#[should_panic]` documents and pins the crash.
#[test]
#[should_panic(expected = "division by zero")]
fn test_all_zero_voting_power_panics_dkg_rounding() {
    // Every genesis validator staked < 1 ether => voting_power truncates to 0.
    let validator_stakes = vec![0u64, 0, 0, 0];

    // On HEAD this call panics ("division by zero") inside
    // compute_profile_fixed_point at rounding/mod.rs:332. It does NOT return.
    let _dkg_rounding = DKGRounding::new(
        &validator_stakes,
        *DEFAULT_SECRECY_THRESHOLD.deref(),
        *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
        Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD.deref()),
    );
}

/// Companion to the above: a *mixed* set where only some validators are sub-ether (here a
/// single non-zero validator among zeros) does NOT divide by zero, but silently produces a
/// degenerate single-power profile. This isolates the all-zero case as the hard crash and
/// documents that a partially-sub-ether genesis is the milder (but still wrong) outcome.
#[test]
fn test_partial_zero_voting_power_does_not_panic_control() {
    // One validator above 1 ether (voting_power 1), the rest truncated to 0.
    let validator_stakes = vec![1u64, 0, 0, 0];
    let dkg_rounding = DKGRounding::new(
        &validator_stakes,
        *DEFAULT_SECRECY_THRESHOLD.deref(),
        *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
        Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD.deref()),
    );
    // Does not panic; stake_sum == 1 != 0. The degenerate weighting is its own concern,
    // but the point here is only that the all-zero case is the division-by-zero trigger.
    println!(
        "partial-zero profile: {:?}",
        dkg_rounding.profile.validator_weights
    );
}

/// Reproduces finding `dkg-recon-threshold-eq-total-weight-zero-ft` (severity HIGH).
///
/// For the small unequal 4-validator weighted set with voting powers
/// `[1_000_000, 1_000_000, 700_000, 1_300_000]` (total 4_000_000, every node strictly
/// below the 1/3 = 1_333_333 BFT bound), the DKG weighted rounding collapses the
/// randomness *reconstruction* threshold to exactly the total weight.
///
/// `DKGRounding::new` accepts the very first profile at `total_weight_min == n == 4`:
/// `compute_profile_fixed_point` with `stake_per_weight == 1_000_000` rounds the ideal
/// weights `[1.0, 1.0, 0.7, 1.3]` to `[1, 1, 1, 1]` (700k -> 1, 1.3M -> 1), giving
/// `delta_up == 0.3`. Then
///   `reconstruct_threshold_in_weights = min(weight_total, ceil(0.5*4M/1M + 0.3) + 1)`
///                                     = min(4, ceil(2.3) + 1) = min(4, 4) = 4.
/// So `reconstruct_threshold_in_weights == total_weight == 4`.
///
/// Consequence: randomness can only be reconstructed once shares of weight >= 4 are
/// collected, i.e. ALL four validators must submit a share every round. A single
/// offline/crashed/Byzantine validator leaves weight 3 < 4, so `rand_store`'s
/// `try_aggregate` never fires, no `Randomness` is decided, and randomness-gated block
/// commit stalls. The randomness subsystem therefore tolerates ZERO faults for a
/// 4-validator BFT set that must tolerate f = 1.
///
/// `is_valid_profile` only checks the reconstruct *stake* ratio
/// (0.5 + 1M*0.6/4M = 0.65 <= 2/3), never an integer weight fault-tolerance margin,
/// so this profile is accepted with no warning.
///
/// This test asserts the liveness-margin invariant
///   `total_weight - reconstruct_threshold_in_weights >= max_single_validator_weight`
/// (the minimum needed to survive any one validator being down). On HEAD it FAILS,
/// demonstrating the defect. The first two assertions pin down the exact arithmetic
/// so the failure cannot be mistaken for a flaky/changed-default issue.
///
/// #[ignore]: FAILS on HEAD by design. Ignored so `cargo test -p aptos-types` stays green; run
/// with `--ignored`. Un-ignore once a weight fault-tolerance margin (or genesis guard) is enforced.
#[test]
#[ignore = "repro of dkg-recon-threshold-eq-total-weight-zero-ft; FAILS until a fault-tolerance \
            margin is enforced. Run with --ignored."]
fn test_recon_threshold_eq_total_weight_zero_fault_tolerance() {
    // The user's probe vector: 4 weighted validators, each strictly under 1/3.
    let validator_stakes = vec![1_000_000u64, 1_000_000, 700_000, 1_300_000];

    let dkg_rounding = DKGRounding::new(
        &validator_stakes,
        *DEFAULT_SECRECY_THRESHOLD.deref(),
        *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
        Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD.deref()),
    );
    let profile = &dkg_rounding.profile;
    println!("probe-vector rounding profile: {:?}", profile);

    // The profile is "valid" by the only acceptance gate, which checks the stake
    // ratio (0.65 <= 2/3) but no integer weight fault-tolerance margin.
    assert!(is_valid_profile(
        profile,
        *DEFAULT_RECONSTRUCT_THRESHOLD.deref()
    ));

    let weights = &profile.validator_weights;
    let total_weight: u64 = weights.iter().sum();
    let recon_threshold = profile.reconstruct_threshold_in_weights;
    let max_single = *weights.iter().max().unwrap();

    // Pin the exact arithmetic so the repro is unambiguous.
    assert_eq!(
        weights,
        &vec![1u64, 1, 1, 1],
        "expected rounded weights [1,1,1,1] for the probe vector"
    );
    assert_eq!(total_weight, 4, "expected total weight 4");

    // The defect: reconstruction threshold collapses to the full total weight.
    assert_eq!(
        recon_threshold, total_weight,
        "DEFECT: reconstruct_threshold_in_weights ({}) == total_weight ({}): \
         randomness requires unanimity, zero crash-fault tolerance",
        recon_threshold, total_weight,
    );

    // The liveness invariant a BFT-of-4 set must satisfy: randomness must still be
    // reconstructible after any single validator is down. This requires the spare
    // weight above threshold to cover the largest single validator's weight.
    //
    // On HEAD: 4 - 4 = 0 < 1  -> this assertion FAILS, demonstrating the bug.
    let spare = total_weight - recon_threshold;
    assert!(
        spare >= max_single,
        "LIVENESS VIOLATION (finding dkg-recon-threshold-eq-total-weight-zero-ft): \
         spare weight above reconstruct threshold = {} (total {} - threshold {}), \
         but the largest single validator weight is {}. With < {} spare weight, a \
         single offline validator (weight {}) drops the collected weight below the \
         reconstruct threshold {}, so randomness can never be decided and the chain \
         stalls. A 4-validator BFT set must tolerate f = 1.",
        spare, total_weight, recon_threshold, max_single, max_single, max_single, recon_threshold,
    );
}

/// Control case showing the defect is specific to the *unequal* small set: an equal
/// 4-validator set `[1M; 4]` rounds to weights `[1,1,1,1]` with `delta_up == 0`, giving
/// `reconstruct_threshold = ceil(0.5*4M/1M) + 1 = 3 < 4`, which DOES leave fault
/// tolerance (spare 1 >= max single weight 1). This passes on HEAD and isolates the
/// unequal-stake `delta_up` inflation as the trigger.
#[test]
fn test_recon_threshold_equal_stakes_has_fault_tolerance_control() {
    let validator_stakes = vec![1_000_000u64; 4];
    let dkg_rounding = DKGRounding::new(
        &validator_stakes,
        *DEFAULT_SECRECY_THRESHOLD.deref(),
        *DEFAULT_RECONSTRUCT_THRESHOLD.deref(),
        Some(*DEFAULT_FAST_PATH_SECRECY_THRESHOLD.deref()),
    );
    let profile = &dkg_rounding.profile;
    let total_weight: u64 = profile.validator_weights.iter().sum();
    let recon_threshold = profile.reconstruct_threshold_in_weights;
    assert_eq!(total_weight, 4);
    assert_eq!(recon_threshold, 3);
    assert!(total_weight - recon_threshold >= 1);
}
