// ref to : https://www.skyzh.dev/blog/2022-01-31-gat-async-trait/

#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(async_fn_in_trait)]
pub trait MyIteratorVersion3 {
    async fn next3(&mut self) -> Option<(&[u8], &[u8])>;
}

use async_trait::async_trait;
use std::{future::Future, io::Write};

#[async_trait]
pub trait MyIteratorVersion1 {
    // Get the next item from the iterator
    async fn next1(&mut self) -> Option<(&[u8], &[u8])>;
}

pub trait MyIteratorVersion2 {
    type NextFuture<'a>: Future<Output = Option<(&'a [u8], &'a [u8])>>
    where
        Self: 'a;

    fn next2<'a>(&'a mut self) -> Self::NextFuture<'a>;
}

pub struct TestIterator {
    idx: usize,
    to_idx: usize,
    key: Vec<u8>,
    value: Vec<u8>,
}

impl TestIterator {
    pub fn new(from_idx: usize, to_idx: usize) -> Self {
        Self {
            idx: from_idx,
            to_idx,
            key: Vec::new(),
            value: Vec::new(),
        }
    }
}

#[async_trait]
impl MyIteratorVersion1 for TestIterator {
    async fn next1(&mut self) -> Option<(&[u8], &[u8])> {
        if self.idx >= self.to_idx {
            return None;
        }

        // Zero-allocation key value manipulation

        self.key.clear();
        write!(&mut self.key, "key_{:05}", self.idx).unwrap();

        self.value.clear();
        write!(&mut self.value, "value_{:05}", self.idx).unwrap();

        self.idx += 1;
        Some((&self.key[..], &self.value[..]))
    }
}

impl MyIteratorVersion2 for TestIterator {
    type NextFuture<'a> = impl Future<Output = Option<(&'a [u8], &'a [u8])>>;

    fn next2<'a>(&'a mut self) -> Self::NextFuture<'a> {
        async move {
            if self.idx >= self.to_idx {
                return None;
            }

            self.key.clear();
            write!(&mut self.key, "key_{:05}", self.idx).unwrap();

            self.value.clear();
            write!(&mut self.value, "value_{:05}", self.idx).unwrap();

            self.idx += 1;
            Some((&self.key[..], &self.value[..]))
        }
    }
}

impl MyIteratorVersion3 for TestIterator {
    async fn next3(&mut self) -> Option<(&[u8], &[u8])> {
        if self.idx >= self.to_idx {
            return None;
        }

        // Zero-allocation key value manipulation

        self.key.clear();
        write!(&mut self.key, "key_{:05}", self.idx).unwrap();

        self.value.clear();
        write!(&mut self.value, "value_{:05}", self.idx).unwrap();

        self.idx += 1;
        Some((&self.key[..], &self.value[..]))
    }
}

pub struct ConcatIterator<Iter: MyIteratorVersion3> {
    iters: Vec<Iter>,
    key: Vec<u8>,
    value: Vec<u8>,
    current_idx: usize,
}

impl<Iter: MyIteratorVersion3> ConcatIterator<Iter> {
    pub fn new(iters: Vec<Iter>) -> Self {
        Self {
            iters,
            current_idx: 0,
            key: Vec::new(),
            value: Vec::new(),
        }
    }
}

impl<Iter: MyIteratorVersion3> MyIteratorVersion3 for ConcatIterator<Iter> {
    async fn next3(&mut self) -> Option<(&[u8], &[u8])> {
        loop {
            if self.current_idx >= self.iters.len() {
                return None;
            }
            let iter = &mut self.iters[self.current_idx];
            match iter.next3().await {
                Some((key, value)) => {
                    self.key.clear();
                    self.key.extend_from_slice(key);
                    self.value.clear();
                    self.value.extend_from_slice(value);

                    break Some((self.key.as_slice(), self.value.as_slice()));
                }
                None => {
                    self.current_idx += 1;
                }
            }
        }
    }
}
// benches/my_iterator_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn test_version1() {
    // start a tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    // create a test iterator
    let mut iter = TestIterator::new(0, 1_000_000);
    // iterate over the iterator
    rt.block_on(async { while let Some((_, _)) = iter.next1().await {} });
}

fn test_version2() {
    // start a tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    // create a test iterator
    let mut iter = TestIterator::new(0, 1_000_000);
    // iterate over the iterator
    rt.block_on(async { while let Some((_, _)) = iter.next2().await {} });
}

fn test_version3() {
    // start a tokio runtime
    let rt = tokio::runtime::Runtime::new().unwrap();
    // create a test iterator
    let mut iter = TestIterator::new(0, 1_000_000);
    // iterate over the iterator
    rt.block_on(async { while let Some((_, _)) = iter.next3().await {} });
}

fn my_iterator_version1(c: &mut Criterion) {
    c.bench_function("my_iterator", |b| b.iter(|| test_version1()));
}

fn my_iterator_version2(c: &mut Criterion) {
    c.bench_function("my_iterator", |b| b.iter(|| test_version2()));
}

fn my_iterator_version3(c: &mut Criterion) {
    c.bench_function("my_iterator", |b| b.iter(|| test_version3()));
}

criterion_group!(
    benches,
    my_iterator_version1,
    my_iterator_version2,
    my_iterator_version3
);

criterion_main!(benches);
