use std::pin::Pin;

use futures::{stream::Stream, Future};
use reqwest::Url;
use scraper::Html;

pub(crate) type PageStream = Pin<Box<dyn Stream<Item = Html>>>;
pub type SpiderResult = Result<(), SpiderError>;
pub type SpiderFuture = Pin<Box<dyn Future<Output = SpiderResult>>>;

#[derive(Debug)]
pub struct SpiderError {}

pub trait Spider {
    fn start_crawl(&self, url: Url) -> PageStream;
    fn on_page(&self, page: Html) -> SpiderFuture;
}
