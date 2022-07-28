use tokio::sync::mpsc;

pub async fn tk_main() {
    let (tx_stop, mut rx_stop) = mpsc::channel::<()>(1);
    let (tx_str, mut rx_str) = mpsc::channel::<String>(10);

    tokio::spawn(async move {
        loop {
            let mut input: String = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            tx_str.send(input).await.unwrap();
        }
    });

    tokio::spawn(async move {
        if let Ok(()) = tokio::signal::ctrl_c().await {
            tx_stop.send(()).await.unwrap();
        }
    });

    loop {
        tokio::select! {
            _ = rx_stop.recv() => {
                println!("press ctrl-c!");
                // do something
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

                std::process::exit(0);
            }
            Some(val) = rx_str.recv() => {
                println!("{}", val.trim());
            }
        }
    }
}
