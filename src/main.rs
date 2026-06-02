use slatedb::object_store::{memory::InMemory, ObjectStore};
use slatedb::config::{FlushOptions, FlushType};
use slatedb::{Db, Error, Settings};
use slatedb_common::metrics::{lookup_metric, DefaultMetricsRecorder, MetricsRecorder};
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let object_store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());
    let metrics_recorder = Arc::new(DefaultMetricsRecorder::new());

    let settings = Settings {
        l0_max_ssts: 1,
        compactor_options: None,
        // Each manifest refresh makes the flusher retry pending L0 dispatch.
        // Since L0 stays full here, each retry increments l0_stall_count.
        manifest_poll_interval: Duration::from_millis(100),
        ..Settings::default()
    };

    let db = Db::builder("test_kv_store", object_store)
        .with_settings(settings)
        .with_metrics_recorder(metrics_recorder.clone() as Arc<dyn MetricsRecorder>)
        .build()
        .await?;

    db.put(b"key-1", b"value-1").await?;
    db.flush_with_options(FlushOptions {
        flush_type: FlushType::MemTable,
    })
    .await?;
    println!("first flush completed; L0 is now full because l0_max_ssts=1");

    db.put(b"key-2", b"value-2").await?;
    let flush = db.flush_with_options(FlushOptions {
        flush_type: FlushType::MemTable,
    });

    match tokio::time::timeout(Duration::from_millis(500), flush).await {
        Err(_) => println!("second flush stalled while L0 is full"),
        Ok(Ok(_)) => println!("unexpected: second flush completed"),
        Ok(Err(err)) => return Err(err),
    }

    for _ in 0..5 {
        tokio::time::sleep(Duration::from_millis(250)).await;
        let count = lookup_metric(&metrics_recorder, "slatedb.db.l0_stall_count").unwrap_or(0);
        println!("slatedb.db.l0_stall_count={count}");
    }

    Ok(())
}
