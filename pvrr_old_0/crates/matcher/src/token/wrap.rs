use super::{Category, Category::*, TokenRef};
use base_tool::text::Text;
use std::fmt::{Debug, Formatter};
use std::ops::Add;

pub(crate) struct Token(Option<TokenRef>);

impl Token {
    fn new(category: Category, text: Text, enclosed: bool) -> Self {
        Self(Some(TokenRef::new(category, text, enclosed)))
    }

    pub(super) fn into_option(self) -> Option<TokenRef> {
        self.0
    }
}

impl Token {
    pub(crate) fn none() -> Self {
        Token(None)
    }

    pub(crate) fn bracket_open(text: Text, enclosed: bool) -> Self {
        Token::new(BracketOpen, text, enclosed)
    }

    pub(crate) fn bracket_closed(text: Text, enclosed: bool) -> Self {
        Token::new(BracketClosed, text, enclosed)
    }

    pub(crate) fn delimiter(text: Text, enclosed: bool) -> Self {
        Token::new(Delimiter, text, enclosed)
    }

    pub(crate) fn unknown(text: Text, enclosed: bool) -> Self {
        Token::new(Unknown, text, enclosed)
    }

    pub(crate) fn is_none(&self) -> bool {
        self.0.is_none()
    }

    pub(crate) fn is_unknown(&self) -> bool {
        self.0.as_ref().map(|it| it.category() == Unknown).unwrap_or(false)
    }

    pub(crate) fn is_open_bracket(&self) -> bool {
        self.0.as_ref().map(|it| it.category() == BracketOpen).unwrap_or(false)
    }

    pub(crate) fn is_closed_bracket(&self) -> bool {
        self.0.as_ref().map(|it| it.category() == BracketClosed).unwrap_or(false)
    }

    pub(crate) fn is_delimiter(&self) -> bool {
        self.0.as_ref().map(|it| it.category() == Delimiter).unwrap_or(false)
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.0.as_ref().map(|it| it.category() != Invalid).unwrap_or(false)
    }

    pub(crate) fn enclosed(&self) -> bool {
        self.0.as_ref().map(|it| it.enclosed()).unwrap_or(false)
    }

    pub(crate) fn to_text(&self) -> Text {
        self.0.as_ref().map(|it| it.text()).unwrap_or_default()
    }

    pub(crate) fn set_unknown(&mut self) {
        self.0.as_mut().map(|it| it.set_category(Unknown));
    }

    pub(crate) fn set_identifier(&mut self) {
        self.0.as_mut().map(|it| it.set_category(Identifier));
    }

    /// 根据提供的 text 进行 deep clone
    pub(crate) fn deep_clone(&self, text: Text) -> Self {
        self.0
            .as_ref()
            .map(|it| Token::new(it.category(), text, it.enclosed()))
            .unwrap_or_else(|| Token::none())
    }
}

impl From<TokenRef> for Token {
    fn from(value: TokenRef) -> Self {
        Token(Some(value))
    }
}

impl From<&TokenRef> for Token {
    fn from(value: &TokenRef) -> Self {
        Token(Some(value.clone()))
    }
}

impl From<Option<&TokenRef>> for Token {
    fn from(value: Option<&TokenRef>) -> Self {
        Token(value.map(|it| it.clone()))
    }
}

impl PartialEq<&TokenRef> for Token {
    fn eq(&self, other: &&TokenRef) -> bool {
        match self.0.as_ref() {
            None => false,
            Some(it) => *it == **other,
        }
    }
}

impl PartialEq<&Token> for Token {
    fn eq(&self, other: &&Token) -> bool {
        *self == **other
    }
}

impl PartialEq<&mut Token> for Token {
    fn eq(&self, other: &&mut Token) -> bool {
        *self == **other
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, other: &Token) -> bool {
        match other.0.as_ref() {
            None => self.0.is_none(),
            Some(rhs) => *self == rhs,
        }
    }
}

impl Add for Token {
    type Output = Token;

    fn add(self, rhs: Token) -> Self::Output {
        let mut rhs = rhs;
        self + &mut rhs
    }
}

impl Add<&mut Token> for Token {
    type Output = Token;

    fn add(self, rhs: &mut Token) -> Self::Output {
        return if let Some(rhs) = rhs.0.as_mut() {
            if let Some(lhs) = self.0.as_ref() {
                let text = lhs.text() + rhs.text();
                let new_token = self.deep_clone(text);
                rhs.set_category(Invalid);
                new_token
            } else {
                let new_token = self.deep_clone(rhs.text());
                rhs.set_category(Invalid);
                new_token
            }
        } else {
            self
        };
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0.as_ref() {
            None => write!(f, "None"),
            Some(it) => write!(f, "{:?}", *it),
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = source.0.clone()
    }
}
