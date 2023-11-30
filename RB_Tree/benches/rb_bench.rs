use criterion::{black_box, criterion_group, criterion_main, Criterion};

use RB_Tree::*;


fn criterion_benchmark(c: &mut Criterion){
    [1000, 10000u32].iter().for_each(|x| {
        let mut rb = new_rb_tree(1);
        c.bench_function(&format!("{}: {}", "insert", x), |b| {
            b.iter(|| {
                (2..=*x).into_iter().for_each(|data| {
                   rb =  insert(&rb, black_box(data));
                });
            });
        });
        c.bench_function(&format!("{}: {}", "find key", x), |b| {
            b.iter(|| {
                (1..=*x/10).into_iter().for_each(|data| {
                   _ =  find_key(rb.clone(), black_box(data));
                });
                
            });
        });
        println!("height: {}",tree_height(&rb));
    });
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);