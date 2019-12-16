#[macro_use]
extern crate bencher;
use bencher::Bencher;

fn benchmark(bench: &mut Bencher)
{
    bench.iter(|| {

    });
}

benchmark_group!(benches, benchmark);
benchmark_main!(benches);
