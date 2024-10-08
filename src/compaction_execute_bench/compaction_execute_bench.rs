#[cfg(feature = "db_bench")]
use slatedb::compaction_execute_bench::run_compaction_execute_bench;

#[cfg(not(feature = "db_bench"))]
fn run_compaction_execute_bench() -> Result<(), slatedb::error::SlateDBError> {
    panic!("db_bench feature not enabled!")
}

fn main() {
    run_compaction_execute_bench().unwrap();
}
