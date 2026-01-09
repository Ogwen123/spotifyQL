use crate::utils::logger::fatal;
use std::sync::mpsc::Sender;
use std::thread;
use tokio::runtime::Runtime;
use warp::Filter;
use warp::http::{HeaderMap, HeaderValue};

pub fn redirect_listener(tx: Sender<String>) {
    // open webserver to listen for auth response
    thread::spawn(move || {
        let rt = Runtime::new().expect("Could not init tokio runtime");
        rt.block_on(async move {
            let mut headers = HeaderMap::new();
            headers.insert("content-type", HeaderValue::from_static("text/html"));

            let redirect_listener = warp::filters::query::raw()
                .map(move |params: String| {
                    if let Err(err) = tx.send(params.clone()) {
                        println!("{}", err);
                        fatal!("Could not send params to channel, see reason above")
                    }
                    // reply with code to immediately close the window on redirect
                    "
                    <!DOCTYPE html>
                     <html>
                     <head>
                     <title>spotifyQL Authentication</title>
                     <script>
                     setTimeout(()=>{}, 1000)
                     window.close()
                     </script>
                     </head>
                       <body>Authenticated Successfully</body>
                     </html>
                     "
                })
                .with(warp::reply::with::headers(headers));

            warp::serve(redirect_listener)
                .run(([127, 0, 0, 1], 5907))
                .await;
        });
    });
}
