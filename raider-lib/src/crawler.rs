use std::{collections::VecDeque, fmt::{Debug}, iter::FromIterator, pin::Pin, result::Result as RResult, task::Poll};

use futures::{stream::Stream, Future};
use reqwest::{Client, Url};
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct CrawlerError;

pub struct ListCrawler {
    list: VecDeque<Url>,
    pos: usize,
}

impl From<url::ParseError> for CrawlerError {
    fn from(_: url::ParseError) -> Self {
        todo!()
    }
}

type Result<T> = RResult<T, CrawlerError>;

impl ListCrawler {
    pub fn from_list<'a, I, T>(v: I) -> Result<Self>
    where
        I: IntoIterator<Item = &'a T>,
        T: AsRef<str> + 'a,
    {
        let list: Vec<Url> = v
            .into_iter()
            .map(|x| x.as_ref().parse::<Url>())
            .collect::<RResult<Vec<Url>, _>>()?;
        Ok(ListCrawler {
            pos: 0,
            list: VecDeque::from(list),
        })
    }
}

impl Stream for ListCrawler {
    type Item = Html;

    fn poll_next(
        mut self: Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut st = self.as_mut();
        let uri = st.list.pop_front();
        match uri {
            Some(x) => {
				todo!()
            }
            None => Poll::Ready(None),
        }
    }
}
