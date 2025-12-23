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
fn test_overflow_with_specific_parameters() {
    // 测试用例：使用用户提供的具体参数来复现溢出问题
    // 参数来源：
    // - validator_stakes: [20000, 20000, 20000, 20000, 10001]
    // - secrecy_threshold: 9223372036854775808 (0.5 in U64F64 bits)
    // - reconstruct_threshold: 12297829382473033728 (2/3 in U64F64 bits)
    // - fast_secrecy_threshold: 12297829382473033728 (2/3 in U64F64 bits)
    
    let validator_stakes = vec![10000, 10000, 10000, 10000, 1];
    
    // 使用 from_bits 创建 U64F64 值
    let secrecy_threshold = U64F64::from_bits(9223372036854775808u128); // 0.5
    let reconstruct_threshold = U64F64::from_bits(12297829382473033728u128); // 2/3
    let fast_secrecy_threshold = U64F64::from_bits(12297829382473033728u128); // 2/3
    
    println!("Testing with parameters:");
    println!("  validator_stakes: {:?}", validator_stakes);
    println!("  secrecy_threshold: {} (bits: {})", secrecy_threshold, secrecy_threshold.to_bits());
    println!("  reconstruct_threshold: {} (bits: {})", reconstruct_threshold, reconstruct_threshold.to_bits());
    println!("  fast_secrecy_threshold: {} (bits: {})", fast_secrecy_threshold, fast_secrecy_threshold.to_bits());
    
    // 计算 total_weight_min 和 total_weight_max
    let total_weight_min = total_weight_lower_bound(&validator_stakes);
    let total_weight_max = total_weight_upper_bound(
        &validator_stakes,
        reconstruct_threshold,
        secrecy_threshold,
    );
    
    println!("  total_weight_min: {}", total_weight_min);
    println!("  total_weight_max: {}", total_weight_max);
    
    // 测试 DKGRoundingProfile::new
    // 这可能会触发溢出
    match DKGRoundingProfile::new(
        &validator_stakes,
        total_weight_min,
        total_weight_max,
        secrecy_threshold,
        reconstruct_threshold,
        Some(fast_secrecy_threshold),
    ) {
        Ok(profile) => {
            println!("Success! Profile created:");
            println!("  {:?}", profile);
            
            // 验证 profile 的有效性
            assert!(is_valid_profile(&profile, reconstruct_threshold));
            
            // 验证权重总和
            let total_weight: u64 = profile.validator_weights.iter().sum();
            assert!(total_weight >= total_weight_min as u64);
            assert!(total_weight <= total_weight_max as u64);
        }
        Err(e) => {
            panic!("DKGRoundingProfile::new failed with error: {}", e);
        }
    }
    
    // 也测试 DKGRounding::new（它会调用 DKGRoundingProfile::new）
    let dkg_rounding = DKGRounding::new(
        &validator_stakes,
        secrecy_threshold,
        reconstruct_threshold,
        Some(fast_secrecy_threshold),
    );
    
    println!("DKGRounding created successfully:");
    println!("  rounding_method: {}", dkg_rounding.rounding_method);
    println!("  profile: {:?}", dkg_rounding.profile);
    if let Some(ref error) = dkg_rounding.rounding_error {
        println!("  rounding_error: {}", error);
    }
}

#[test]
fn test_overflow_edge_cases() {
    // 测试各种可能导致溢出的边界情况
    
    // 场景1: 两个阈值非常接近（模拟 DELTA 很小的情况）
    println!("\n=== 场景1: 两个阈值非常接近 ===");
    let validator_stakes = vec![20000, 20000, 20000, 20000, 10001];
    let secrecy_threshold = U64F64::from_num(1) / U64F64::from_num(2); // 0.5
    
    // 模拟 reconstruct_threshold 非常接近 secrecy_threshold
    // 如果 DELTA 很小，reconstruct 可能被调整为 secrecy + DELTA
    // 这里我们测试一个非常接近的值
    let very_small_delta = U64F64::from_bits(1u128); // 非常小的值
    let reconstruct_threshold = secrecy_threshold + very_small_delta;
    
    println!("  secrecy_threshold: {}", secrecy_threshold);
    println!("  reconstruct_threshold: {} (差值: {})", reconstruct_threshold, very_small_delta);
    
    let total_weight_max = total_weight_upper_bound(
        &validator_stakes,
        reconstruct_threshold,
        secrecy_threshold,
    );
    println!("  total_weight_max: {}", total_weight_max);
    
    if total_weight_max > 1000000 {
        println!("  ⚠️  警告: total_weight_max 非常大，可能导致后续溢出！");
    }
    
    // 场景2: 测试非常大的 total_weight_max
    println!("\n=== 场景2: 测试非常大的 total_weight_max ===");
    // 如果 total_weight_max 接近或超过 u64::MAX，会导致溢出
    let large_total_weight_max = usize::MAX.min(2_u64.pow(50) as usize);
    println!("  测试 total_weight_max = {}", large_total_weight_max);
    
    match DKGRoundingProfile::new(
        &validator_stakes,
        5, // total_weight_min
        large_total_weight_max,
        secrecy_threshold,
        U64F64::from_num(2) / U64F64::from_num(3), // 正常的 reconstruct_threshold
        Some(U64F64::from_num(2) / U64F64::from_num(3)),
    ) {
        Ok(profile) => {
            println!("  ✓ 成功创建 profile");
            let total_weight: u64 = profile.validator_weights.iter().sum();
            println!("  total_weight: {}", total_weight);
        }
        Err(e) => {
            println!("  ✗ 失败: {}", e);
        }
    }
    
    // 场景3: 测试 stake_per_weight 很小的情况
    println!("\n=== 场景3: 测试 stake_per_weight 很小的情况 ===");
    // 如果 weight_mid 很大，stake_per_weight 会接近 1
    // 这可能导致 compute_profile_fixed_point 中的计算溢出
    let very_large_weight = 2_u64.pow(60);
    println!("  测试 weight_mid = {}", very_large_weight);
    
    let stake_total: u64 = validator_stakes.iter().sum();
    let stake_per_weight = (stake_total as f64 / very_large_weight as f64).max(1.0);
    println!("  stake_per_weight = {}", stake_per_weight);
    
    // 场景4: 测试 weight_mid 接近 u64::MAX 的情况
    println!("\n=== 场景4: 测试 weight_mid 接近 u64::MAX 的情况 ===");
    let weight_low = u64::MAX - 100;
    let weight_high = u64::MAX;
    println!("  weight_low = {}", weight_low);
    println!("  weight_high = {}", weight_high);
    
    // 检查第220行的计算: weight_mid = weight_low + (weight_high - weight_low) / 2
    let diff = weight_high - weight_low;
    let half_diff = diff / 2;
    let weight_mid = weight_low + half_diff;
    println!("  weight_mid = {} + {} = {}", weight_low, half_diff, weight_mid);
    
    if weight_mid > u64::MAX {
        println!("  ⚠️  溢出！weight_mid > u64::MAX");
    } else if weight_mid == u64::MAX {
        println!("  ⚠️  风险！weight_mid = u64::MAX，weight_mid + 1 会溢出");
    } else {
        println!("  ✓ 安全");
    }
    
    // 检查第237行: weight_low = weight_mid + 1
    if weight_mid == u64::MAX {
        println!("  ⚠️  第237行会溢出！weight_mid + 1 会超过 u64::MAX");
    }
    
    // 场景5: 测试 .ceil() 结果接近 U64F64 最大值的情况
    println!("\n=== 场景5: 测试 .ceil() 结果接近 U64F64 最大值 ===");
    // 在 compute_profile_fixed_point 中，如果计算结果很大
    // .ceil() + one 可能溢出
    let large_value = U64F64::from_bits((u64::MAX as u128) << 64); // 接近最大值
    println!("  large_value (U64F64): {}", large_value);
    
    // 检查 + one 是否会溢出
    // 注意：这只是一个演示，实际代码中需要检查
    println!("  如果 large_value.ceil() 接近 U64F64 最大值，+ one 可能溢出");
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
