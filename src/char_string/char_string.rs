use std::{convert::TryFrom, ops};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CharString {
    /// 存储 Unicode 标量值，保证合法性和确定性
    inner: Vec<char>,
    /// 缓存 UTF-8 字节长度（避免重复计算）
    byte_len: usize,
}

impl CharString {
    pub fn slice(&self, range: ops::Range<usize>) -> String {
        self.inner[range].iter().collect()
    }

    // --------------------- 核心构造方法 ---------------------
    pub fn from_chars_unchecked(chars: Vec<char>) -> Self {
        let byte_len = chars.iter().map(|c| c.len_utf8()).sum();
        Self {
            inner: chars,
            byte_len,
        }
    }

    /// 预分配内存（适用于高频修改场景）
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            byte_len: 0,
        }
    }

    pub fn push(&mut self, c: char) {
        self.byte_len += c.len_utf8();
        self.inner.push(c);
    }

    pub fn extend<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        let (min, _) = iter.size_hint();
        self.inner.reserve(min);
        for c in iter {
            self.push(c);
        }
    }

    pub fn into_string(self) -> String {
        let mut s = String::with_capacity(self.byte_len);
        s.extend(self.inner);
        s
    }

    pub fn as_str(&self) -> &str {
        // 利用内存布局兼容性实现零拷贝
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                self.inner.as_ptr() as *const u8,
                self.byte_len,
            ))
        }
    }

    pub fn get(&self, i: usize) -> Option<char> {
        for tuple in self.inner.iter().enumerate() {
            if tuple.0 == i {
                return Some(*tuple.1);
            }
        }

        None
    }
}

impl From<String> for CharString {
    fn from(s: String) -> Self {
        let byte_len = s.len();
        let chars = s.chars().collect();
        Self {
            inner: chars,
            byte_len,
        }
    }
}

impl<'a> From<&'a str> for CharString {
    fn from(s: &'a str) -> Self {
        Self::from_chars_unchecked(s.chars().collect())
    }
}

impl ops::Deref for CharString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl TryFrom<CharString> for String {
    type Error = std::convert::Infallible;

    fn try_from(value: CharString) -> Result<Self, Self::Error> {
        Ok(value.into_string())
    }
}

impl ops::Index<ops::Range<usize>> for CharString {
    type Output = str;

    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        &self.as_str()[index]
    }
}

impl IntoIterator for CharString {
    type Item = char;
    type IntoIter = std::vec::IntoIter<char>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
