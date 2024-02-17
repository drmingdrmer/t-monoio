use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let mut rt = monoio::RuntimeBuilder::<monoio::FusionDriver>::new()
            .enable_all()
            .build()
            .expect("Failed building the Runtime");

        rt.block_on(async move {
            let fu = async move {
                // println!("inner-fu: ready");
                1u64
            };

            let handle = monoio::spawn(fu);
            tx.send(handle).unwrap();

            monoio::time::sleep(Duration::from_millis(1_000)).await;
            println!("outer-fu: after sending handle and sleep");
        });
    });

    let handle = rx.recv().unwrap();

    let mut rt = monoio::RuntimeBuilder::<monoio::FusionDriver>::new()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    rt.block_on(async move {
        println!("joiner: before handle.await");
        let got = handle.await;
        println!("joiner: after handle.await: {:?}", got);
    });
}
