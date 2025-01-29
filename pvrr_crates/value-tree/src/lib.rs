mod json_support;
mod xml_support;

use html5ever::{ns, Attribute, QualName};
use scraper::node::Element;
use scraper::{ElementRef, Html, Node};

fn root() -> Node {
    let name = QualName::new(None, ns!(html), "root".into());
    let attr = vec![Attribute {
        name: QualName::new(None, ns!(), "class".into()),
        value: "root".into(),
    }];
    Node::Element(Element::new(name, attr))
}

struct DOM(Html);

impl DOM {
    pub fn parse_html(html: &str) -> Self {
        Self(Html::parse_document(html))
    }

    pub fn root_element(&self) -> ElementRef {
        self.0.root_element()
    }
}
