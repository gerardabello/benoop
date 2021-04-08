use reqwest::StatusCode;
use std::time::{Duration, Instant};

use crossbeam_channel::bounded;

use crate::config::{Config, RequestConfig};
use crate::error::Error;
use crate::reporter::Results;
use reqwest::get;
use tokio::spawn;

async fn make_request(request_config: &RequestConfig) -> Result<(), Error> {
    let expected = vec![StatusCode::OK, StatusCode::NO_CONTENT, StatusCode::CREATED];

    let resp = get(&request_config.url)
        .await
        .map_err(|e| Error::RequestError {
            request: request_config.clone(),
            source: e,
        })?;

    let status_code = resp.status();

    if !expected.contains(&status_code) {
        return Err(Error::UnexpectedStatusCode {
            status_code,
            expected,
            request: request_config.clone(),
        });
    }

    Ok(())
}

pub async fn run_config(config: &'static Config) -> Results {
    let (work_sender, work_reciever) = bounded::<&RequestConfig>(config.total);
    let (result_sender, result_reciever) = bounded::<(&RequestConfig, Duration)>(config.total);

    let handles: Vec<tokio::task::JoinHandle<()>> = (0..config.concurrent)
        .map(|_| {
            let lwr = work_reciever.clone();
            let lrs = result_sender.clone();
            spawn(async move {
                while let Ok(req) = lwr.recv() {
                    let start = Instant::now();
                    // In case a request fails, exit immidiately
                    if let Err(e) = make_request(&req).await {
                        // TODO stop theads and return Result from run_config instead of
                        // process::exit.
                        println!("{}", e);
                        std::process::exit(1)
                    }
                    lrs.send((req, start.elapsed()))
                        .expect("channel should be open");
                }
            })
        })
        .collect();

    for _ in 0..config.total {
        work_sender
            .send(&config.get_random_request())
            .expect("channel should be open");
    }

    // Drop sender so .recv() does not block after it gets emptied. It will return an Err and the
    // threads will stop (see documentation for crossbeam_channel:bounded);
    drop(work_sender);

    let start = Instant::now();
    for handle in handles {
        handle.await.expect("threads should exit correctly");
    }

    let total_duration = start.elapsed();

    let mut results: Results = Results {
        total: total_duration,
        config: config.to_owned(),
        times: vec![vec![]; config.requests.len()],
    };

    while let Ok(dur) = result_reciever.try_recv() {
        let index = config
            .requests
            .iter()
            .position(|rc| rc == dur.0)
            .expect("The duration must come from one of the configured requests");
        results.times[index].push(dur.1);
    }

    results
}
