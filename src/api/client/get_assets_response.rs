use lazy_regex::regex;

use crate::model::{Asset, AssetClass};

#[derive(Deserialize)]
pub struct GetAssetsResponse {
    data: Vec<Data>,
    links: Links,
}

impl GetAssetsResponse {
    pub fn next_page(&self) -> Option<usize> {
        if let Some(next) = &self.links.next {
            let rgx = regex!(r"page=([0-9]+)");
            rgx.captures(next).and_then(|captures| {
                captures
                    .get(1)
                    .map(|capture| capture.as_str().parse().unwrap())
            })
        } else {
            None
        }
    }

    pub fn into_assets(self, class: AssetClass) -> Vec<Asset> {
        let mut assets = Vec::with_capacity(self.data.len());
        for data in self.data.into_iter() {
            assets.push(Asset {
                id: data.id,
                name: data.attributes.name,
                pid: data.attributes.pid,
                symbol: data.attributes.symbol,
                r#type: class,
            })
        }
        assets
    }
}

#[derive(Deserialize)]
struct Links {
    next: Option<String>,
}

#[derive(Deserialize)]
struct Data {
    id: String,
    attributes: Attributes,
}

#[derive(Deserialize)]
struct Attributes {
    symbol: String,
    name: String,
    pid: String,
}

#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_parse_next_page() {
        let response = GetAssetsResponse {
            data: vec![],
            links: Links {
                next: Some("?page=18&page_size=2".to_string()),
            },
        };

        assert_eq!(response.next_page(), Some(18));

        let response = GetAssetsResponse {
            data: vec![],
            links: Links { next: None },
        };

        assert!(response.next_page().is_none());
    }
}
