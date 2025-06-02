use std::{future::Future, process::Output};

use serde::Serialize;

use crate::{podsync, subscription::SubscriptionChangesToClient};

use opml::OPML;

/* Building an OPML string of all subscriptions
Taking example of AntennaPod OPML exported feed:
*/
async fn get_info(rss_feed: &str) -> (String, String) {
    todo!()
}

async fn subscriptions_to_opml<F, B>(f: F) -> impl warp::Reply
where
    F: Future<Output = podsync::Result<SubscriptionChangesToClient>>,
{
    match f.await {
        Ok(body) => {
            let subscription_infos: Vec<(String, String)> =
                body.add.iter().map(|url| get_info(url)).collect();
            let mut new_opml = OPML::default();
            for info in subscription_infos {
                new_opml.add_feed(&info.0, &info.1);
            }
            new_opml.to_string().unwrap()
        }
        Err(_) => todo!("Here be error processing logic"),
    }
}
