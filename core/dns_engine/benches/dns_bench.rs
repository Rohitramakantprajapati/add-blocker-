use std::fs;
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use dns_engine::{blocklist::Blocklist, BlockDecision};

fn prepare_db() -> PathBuf {
    let dir = std::env::temp_dir();
    let path = dir.join("voidblock-dns-bench.db");
    if path.exists() {
        if let Err(error) = fs::remove_file(&path) {
            let _ = error;
        }
    }
    if let Ok(conn) = rusqlite::Connection::open(&path) {
        assert!(conn
            .execute_batch("CREATE TABLE blocked (domain TEXT PRIMARY KEY NOT NULL);")
            .is_ok());
        assert!(conn
            .execute(
                "INSERT INTO blocked(domain) VALUES (?1)",
                rusqlite::params!["ads.example.com"],
            )
            .is_ok());
    }
    path
}

fn bench_dns(c: &mut Criterion) {
    let db_path = prepare_db();
    let blocklist = match Blocklist::open(&db_path) {
        Ok(blocklist) => blocklist,
        Err(_) => return,
    };
    let queries = vec!["github.com", "ads.example.com", "sub.doubleclick.net"];

    let mut group = c.benchmark_group("dns_block_decision");
    group.bench_function(BenchmarkId::new("lookup", queries.len()), |b| {
        b.iter(|| {
            for domain in &queries {
                let decision = match blocklist.check(domain) {
                    Ok(decision) => decision,
                    Err(_) => BlockDecision::Allow,
                };
                criterion::black_box(decision);
            }
        });
    });
    group.finish();

    if let Ok(decision) = blocklist.check("ads.example.com") {
        assert!(matches!(decision, BlockDecision::Block));
    }
}

criterion_group!(benches, bench_dns);
criterion_main!(benches);