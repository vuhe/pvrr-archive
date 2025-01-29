mod attributes;
mod node_tag;
mod str_value;
mod tag_name;

#[derive(Debug)]
pub(crate) enum Node {
    Root,
    Ignore,
    Tag(node_tag::NodeTag),
    Val(str_value::StrValue),
}
