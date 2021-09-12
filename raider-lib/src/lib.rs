use std::marker::PhantomData;

use reqwest::Url;
use scraper::Html;
use spider::Spider;
mod crawler;
mod spider;

pub struct EngineError {}

pub async fn run<T>(url: Url) -> Result<(), EngineError>
where
T: Spider + Default {
		let st = T::default();
	run_with(T::default(),url)
	}

pub async fn run_with(s: impl Spider, u: Url) -> Result<(), EngineError> {
	todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
