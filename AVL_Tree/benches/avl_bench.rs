use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use AVL_Tree::*;


fn criterion_benchmark(c: &mut Criterion){
    let mut group = c.benchmark_group("AVL Benches");
    group.measurement_time(Duration::from_secs(5));
    [10000, 40000, 70000, 100000, 130000u32].iter().for_each(|x| {
        let mut avl = new_avl_tree(1);
        group.bench_function(&format!("{}: {}", "insert", x), |b| {
            b.iter(|| {
                (2..=*x).into_iter().for_each(|data| {
                    avl =  insert(&avl, black_box(data));
                });
            });
        });
        group.bench_function(&format!("{}: {}", "find key", x), |b| {
            b.iter(|| {
                (1..=*x/10).into_iter().for_each(|data| {
                   _ =  find_key(avl.clone(), black_box(data));
                });
                
            });
        });
        println!("height: {}",tree_height(&avl));
    });
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);