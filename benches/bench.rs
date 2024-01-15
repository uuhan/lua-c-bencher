use criterion::{criterion_group, criterion_main, Criterion};
use luajit_c_bencher as lib;
use mlua::prelude::*;

fn criterion_benchmark(crt: &mut Criterion) {
    crt.bench_function("lua vm create", |b| {
        b.iter(|| {
            let _ = Lua::new();
        })
    });

    crt.bench_function("lua.export.add(1,1)", |b| {
        let lua = Lua::new();
        let calc: LuaFunction = lua
            .load("require('lua.export').add")
            .eval()
            .expect("lua load");

        b.iter(|| {
            let result = calc.call::<_, i32>((1, 1)).expect("lua run");
            assert_eq!(2, result);
        })
    });

    crt.bench_function("c.add(1+1)", |b| {
        b.iter(|| unsafe {
            let result = lib::c::add(1, 1);
            assert_eq!(2, result);
        })
    });

    crt.bench_function("rust.add(1+1)", |b| {
        b.iter(|| {
            let result = lib::add(1, 1);
            assert_eq!(2, result);
        })
    });

    crt.bench_function("lua.export.fib(10)", |b| {
        let lua = Lua::new();
        let calc: LuaFunction = lua
            .load("require('lua.export').fib")
            .eval()
            .expect("lua load");

        b.iter(|| {
            let _ = calc.call::<_, i32>(10).expect("lua run");
        })
    });

    crt.bench_function("c.fib(10)", |b| {
        b.iter(|| unsafe {
            let _ = lib::c::fib(10);
        })
    });

    crt.bench_function("rust.fib(10)", |b| {
        b.iter(|| {
            let _ = lib::fib(10);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
