# Benchmarking your application/library with Criterion 

Add Criterion as dev-dependency, create a folder `benches` with benchmarks, here `my_benchmarks.rs` and `comparison_benchmark.rs`.

To get graphics with the benchmark results (in the generated html page), install gnuplot on your system.

Run `cargo bench` to run the benchmark.

The html report is generated in target/criterion/<name of your benchmark>.

More details on Criterion can be found in the [User Guide](https://bheisler.github.io/criterion.rs/book/criterion_rs.html).

Regression testing can also be done by running `cargo bench -- --save-baseline <name-of-baseline>`, which will save the baseline results in `baseline.json`. Unfortunately, I did not get that to work yet.

    cargo bench -- --save-baseline initial                     
    Finished `bench` profile [optimized] target(s) in 0.10s
     Running unittests src/lib.rs (target/release/deps/criterion_benchmark-c2076c60856bf728)
    error: Unrecognized option: 'save-baseline'
    error: bench failed, to rerun pass `--lib`

By passing `--baseline <name-of-baseline>` to next runs, they will be compared to this baseline.
