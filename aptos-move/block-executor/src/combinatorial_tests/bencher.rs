// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]

use crate::{
    code_cache_global_manager::AptosModuleCacheManagerGuard,
<<<<<<< HEAD:aptos-move/block-executor/src/proptest_types/bencher.rs
    executor::BlockExecutor,
    proptest_types::{
        baseline::BaselineOutput,
        types::{
            KeyType, MockOutput, MockTask, MockTransaction, TransactionGen, TransactionGenParams,
        },
=======
    combinatorial_tests::{
        baseline::BaselineOutput,
        mock_executor::{MockOutput, MockTask},
        types::{KeyType, MockTransaction, TransactionGen, TransactionGenParams},
>>>>>>> aptos-node-v1.36.8-hotfix:aptos-move/block-executor/src/combinatorial_tests/bencher.rs
    },
    executor::BlockExecutor,
    txn_commit_hook::NoOpTransactionCommitHook,
    txn_provider::default::DefaultTxnProvider,
};
use aptos_types::{
<<<<<<< HEAD:aptos-move/block-executor/src/proptest_types/bencher.rs
    block_executor::config::BlockExecutorConfig, contract_event::TransactionEvent,
    state_store::MockStateView,
=======
    block_executor::{
        config::BlockExecutorConfig, transaction_slice_metadata::TransactionSliceMetadata,
    },
    contract_event::TransactionEvent,
    state_store::MockStateView,
    transaction::AuxiliaryInfo,
>>>>>>> aptos-node-v1.36.8-hotfix:aptos-move/block-executor/src/combinatorial_tests/bencher.rs
};
use criterion::{BatchSize, Bencher as CBencher};
use num_cpus;
use proptest::{
    arbitrary::Arbitrary,
    collection::vec,
    prelude::*,
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
};
use std::{fmt::Debug, hash::Hash, marker::PhantomData, sync::Arc};

pub struct Bencher<K, V, E> {
    transaction_size: usize,
    transaction_gen_param: TransactionGenParams,
    universe_size: usize,
    phantom: PhantomData<(K, V, E)>,
}

pub(crate) struct BencherState<
    K: Hash + Clone + Debug + Eq + PartialOrd + Ord + Send + Sync + 'static,
    E: Send + Sync + Debug + Clone + TransactionEvent + 'static,
> {
<<<<<<< HEAD:aptos-move/block-executor/src/proptest_types/bencher.rs
    txns_provider: DefaultTxnProvider<MockTransaction<KeyType<K>, E>>,
=======
    txns_provider: DefaultTxnProvider<MockTransaction<KeyType<K>, E>, AuxiliaryInfo>,
>>>>>>> aptos-node-v1.36.8-hotfix:aptos-move/block-executor/src/combinatorial_tests/bencher.rs
    baseline_output: BaselineOutput<KeyType<K>>,
}

impl<K, V, E> Bencher<K, V, E>
where
    K: Hash + Clone + Debug + Eq + Send + Sync + PartialOrd + Ord + Arbitrary + 'static,
    V: Clone + Eq + Send + Sync + Arbitrary + 'static,
    E: Send + Sync + Debug + Clone + TransactionEvent + 'static,
    Vec<u8>: From<V>,
{
    pub fn new(transaction_size: usize, universe_size: usize) -> Self {
        Self {
            transaction_size,
            transaction_gen_param: TransactionGenParams::default(),
            universe_size,
            phantom: PhantomData,
        }
    }

    pub fn bench(&self, key_strategy: &impl Strategy<Value = K>, bencher: &mut CBencher) {
        bencher.iter_batched(
            || {
                BencherState::<K, E>::with_universe::<V>(
                    vec(key_strategy, self.universe_size),
                    self.transaction_size,
                    self.transaction_gen_param,
                )
            },
            |state| state.run(),
            // The input here is the entire list of signed transactions, so it's pretty large.
            BatchSize::LargeInput,
        )
    }
}

impl<K, E> BencherState<K, E>
where
    K: Hash + Clone + Debug + Eq + Send + Sync + PartialOrd + Ord + 'static,
    E: Send + Sync + Debug + Clone + TransactionEvent + 'static,
{
    /// Creates a new benchmark state with the given account universe strategy and number of
    /// transactions.
    pub(crate) fn with_universe<
        V: Into<Vec<u8>> + Clone + Eq + Send + Sync + Arbitrary + 'static,
    >(
        universe_strategy: impl Strategy<Value = Vec<K>>,
        num_transactions: usize,
        transaction_params: TransactionGenParams,
    ) -> Self {
        let mut runner = TestRunner::default();
        let key_universe = universe_strategy
            .new_tree(&mut runner)
            .expect("creating a new value should succeed")
            .current();

        let transaction_gens = vec(
            any_with::<TransactionGen<V>>(transaction_params),
            num_transactions,
        )
        .new_tree(&mut runner)
        .expect("creating a new value should succeed")
        .current();

        let transactions: Vec<_> = transaction_gens
            .into_iter()
            .map(|txn_gen| txn_gen.materialize(&key_universe))
            .collect();
<<<<<<< HEAD:aptos-move/block-executor/src/proptest_types/bencher.rs
        let txns_provider = DefaultTxnProvider::new(transactions.clone());
=======
        let txns_provider: DefaultTxnProvider<MockTransaction<KeyType<K>, E>, AuxiliaryInfo> =
            DefaultTxnProvider::new_without_info(transactions.clone());
>>>>>>> aptos-node-v1.36.8-hotfix:aptos-move/block-executor/src/combinatorial_tests/bencher.rs

        let baseline_output = BaselineOutput::generate(txns_provider.get_txns(), None);

        Self {
<<<<<<< HEAD:aptos-move/block-executor/src/proptest_types/bencher.rs
            txns_provider: DefaultTxnProvider::new(transactions),
=======
            txns_provider: DefaultTxnProvider::new_without_info(transactions),
>>>>>>> aptos-node-v1.36.8-hotfix:aptos-move/block-executor/src/combinatorial_tests/bencher.rs
            baseline_output,
        }
    }

    pub(crate) fn run(self) {
        let state_view = MockStateView::empty();

        let executor_thread_pool = Arc::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_cpus::get())
                .build()
                .unwrap(),
        );

        let config = BlockExecutorConfig::new_no_block_limit(num_cpus::get());
        let mut guard = AptosModuleCacheManagerGuard::none();

        let output = BlockExecutor::<
            MockTransaction<KeyType<K>, E>,
            MockTask<KeyType<K>, E>,
            MockStateView<KeyType<K>>,
            NoOpTransactionCommitHook<MockOutput<KeyType<K>, E>, usize>,
<<<<<<< HEAD:aptos-move/block-executor/src/proptest_types/bencher.rs
            DefaultTxnProvider<MockTransaction<KeyType<K>, E>>,
        >::new(config, executor_thread_pool, None)
        .execute_transactions_parallel(&self.txns_provider, &state_view, &mut guard);
=======
            DefaultTxnProvider<MockTransaction<KeyType<K>, E>, AuxiliaryInfo>,
            AuxiliaryInfo,
        >::new(config, executor_thread_pool, None)
        .execute_transactions_parallel(
            &self.txns_provider,
            &state_view,
            &TransactionSliceMetadata::unknown(),
            &mut guard,
        );
>>>>>>> aptos-node-v1.36.8-hotfix:aptos-move/block-executor/src/combinatorial_tests/bencher.rs

        self.baseline_output.assert_parallel_output(&output);
    }
}
