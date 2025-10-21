use std::{borrow::Cow, fmt, marker::PhantomData};

use serde::{
    Deserialize, Deserializer,
    de::{DeserializeSeed, MapAccess, Visitor},
};

use crate::page::attributes::Attributes;

pub struct PageSeed<'a, T> {
    pub root: &'a str,
    pub content: ContentSeed<'a, T>,
}

pub struct ContentSeed<'a, T> {
    pub content: &'a str,
    _marker: PhantomData<T>,
}

impl<'a, T> PageSeed<'a, T> {
    pub fn new(root: &'a str, content: &'a str) -> Self {
        Self {
            root,
            content: ContentSeed {
                content,
                _marker: PhantomData,
            },
        }
    }
}

struct TopVisitor<'a, T> {
    root: &'a str,
    content: ContentSeed<'a, T>,
}

struct ContentVisitor<'a, T> {
    content: &'a str,
    _m: PhantomData<T>,
}

impl<'de, 'a, T: Deserialize<'de>> Visitor<'de> for TopVisitor<'a, T> {
    type Value = super::Page<T>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a map like { root: { \"@attr\": {...}, content: [...] } }")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut root_found = None;

        while let Some(key) = map.next_key::<Cow<'de, str>>()? {
            if key == self.root {
                let inner = map.next_value_seed(self.content)?;
                root_found = Some(inner);
                break;
            } else {
                let _: serde::de::IgnoredAny = map.next_value()?;
            }
        }

        root_found.ok_or_else(|| serde::de::Error::custom("missing root"))
    }
}

impl<'de, 'a, T: Deserialize<'de>> DeserializeSeed<'de> for PageSeed<'a, T> {
    type Value = super::Page<T>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_map(TopVisitor {
            root: self.root,
            content: self.content,
        })
    }
}

impl<'de, 'a, T: Deserialize<'de>> Visitor<'de> for ContentVisitor<'a, T> {
    type Value = super::Page<T>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a map like { \"@attr\": {...}, content: [...] }")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut attr_found = None;
        let mut items_found = None;

        while let Some(key) = map.next_key::<Cow<'de, str>>()? {
            if key == "@attr" {
                let inner: Attributes = map.next_value()?;
                attr_found = Some(inner);
            } else if key == self.content {
                let inner: Vec<T> = map.next_value()?;
                items_found = Some(inner);
            } else {
                let _: serde::de::IgnoredAny = map.next_value()?;
            }

            if attr_found.is_some() && items_found.is_some() {
                break;
            }
        }

        let attr = attr_found.ok_or_else(|| serde::de::Error::custom("missing @attr"))?;
        let items = items_found.ok_or_else(|| serde::de::Error::custom("missing content key"))?;

        Ok(super::Page { attr, items })
    }
}

impl<'de, 'a, T: Deserialize<'de>> DeserializeSeed<'de> for ContentSeed<'a, T> {
    type Value = super::Page<T>;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        deserializer.deserialize_map(ContentVisitor {
            content: self.content,
            _m: PhantomData,
        })
    }
}
