use warp::Filter;
use std::sync::mpsc::Sender;
use std::thread;
use tokio::runtime::Runtime;
use crate::utils::logger::fatal;

pub fn redirect_listener(tx: Sender<String>) {
    // open webserver to listen for auth response
    thread::spawn(move || {
        let rt = Runtime::new().expect("Could not init tokio runtime");
        rt.block_on(async move {
            let redirect_listener = warp::filters::query::raw()
                .map(move |params: String| {
                    if let Err(err) = tx.send(params.clone()) {
                        println!("{}", err);
                        fatal!("Could not send params to channel, see reason above")
                    }
                    params
                });

            warp::serve(redirect_listener).run(([127, 0, 0, 1], 5907)).await;
        });
    });
}
