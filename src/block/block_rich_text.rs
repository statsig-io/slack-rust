//! A section is one of the most flexible blocks available

use crate::block::block_object::TextBlockObject;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// A section is one of the most flexible blocks available.  
/// See: <https://api.slack.com/reference/block-kit/blocks#rich_text>
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Clone)]
pub struct RichTextBlock {
    #[serde(rename = "type")]
    pub type_filed: String,
    // TODO: Need to implement this fully
    pub elements: Vec<String>,
    pub block_id: Option<String>,
}

impl RichTextBlock {
    pub fn builder() -> RichTextBlockBuilder {
        RichTextBlockBuilder::new()
    }
}

#[derive(Debug, Default)]
pub struct RichTextBlockBuilder {
    pub text: Option<TextBlockObject>,
    pub elements: Vec<String>,
    pub block_id: Option<String>,
}

impl RichTextBlockBuilder {
    pub fn new() -> RichTextBlockBuilder {
        RichTextBlockBuilder {
            ..Default::default()
        }
    }
    pub fn elements(self, _elements: Vec<String>) -> RichTextBlockBuilder {
        unimplemented!();
    }
    pub fn block_id(mut self, block_id: String) -> RichTextBlockBuilder {
        self.block_id = Some(block_id);
        self
    }
    pub fn build(self) -> RichTextBlock {
        RichTextBlock {
            type_filed: "rich_text".to_string(),
            elements: self.elements,
            block_id: self.block_id,
        }
    }
}
