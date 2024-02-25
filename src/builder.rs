use crate::Uwu;

/// A builder for the `Uwu` instance.
///
/// Quick start:
/// ```
/// uwu::UwuBuilder::new()
///     .lowercase()
///     .expressions()
///     .w_replace()
///     .stutter(4)
///     .emojis(1)
///     .build();
/// ```
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct UwuBuilder {
    /// Enables the lowercase feature, e.g. 'Hello' becomes 'hello'. Beware that the other features
    /// may misbehave if this feature is disabled.
    pub lowercase: bool,
    /// Enables expression replacement, e.g. 'what' becomes 'nani'.
    pub expressions: bool,
    /// Enables replacement of 'l' and 'r' with 'w', e.g. 'lovely' becomes 'wovewy'.
    pub w_replace: bool,
    /// Enables stutter, e.g. 'hello' becomes 'h-hello'.
    pub stutter: bool,
    /// Specifies how frequent the stutter is. A value of 1 will add stutter to every word, whereas
    /// a value of 2 will add stutter every 2 words on average.
    pub stutter_chance: u8,
    /// Enables adding emojis after punctuation, e.g. 'goodbye.' becomes 'goodbye. OwO'.
    pub emojis: bool,
    /// Specifies how frequently emojis are added. A value of 1 will add emojis after every
    /// punctuation, whereas a value of 2 will add emojis every 2 punctuation marks on average.
    pub emojis_chance: u8,
}

impl UwuBuilder {
    /// Create a new instance of a builder for the `Uwu` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables the lowercase feature, e.g. 'Hello' becomes 'hello'.
    pub fn lowercase(&mut self) -> &mut Self {
        self.lowercase = true;
        self
    }

    /// Enables expression replacement, e.g. 'what' becomes 'nani'.
    pub fn expressions(&mut self) -> &mut Self {
        self.expressions = true;
        self
    }

    /// Enables replacement of 'l' and 'r' with 'w', e.g. 'lovely' becomes 'wovewy'.
    pub fn w_replace(&mut self) -> &mut Self {
        self.w_replace = true;
        self
    }

    /// Enables stutter, e.g. 'hello' becomes 'h-hello'.
    pub fn stutter(&mut self, chance: u8) -> &mut Self {
        self.stutter = true;
        self.stutter_chance = chance;
        self
    }

    /// Enables adding emojis after punctuation, e.g. 'goodbye.' becomes 'goodbye. OwO'.
    pub fn emojis(&mut self, chance: u8) -> &mut Self {
        self.emojis = true;
        self.emojis_chance = chance;
        self
    }

    /// Builds a new `Uwu` instance from the arguments set in this builder.
    pub fn build(&self) -> Uwu {
        Uwu::from(self)
    }
}

impl From<&UwuBuilder> for Uwu {
    fn from(builder: &UwuBuilder) -> Self {
        Self {
            lowercase: builder.lowercase,
            expressions: builder.expressions,
            w_replace: builder.w_replace,
            stutter: builder.stutter,
            stutter_chance: builder.stutter_chance,
            emojis: builder.emojis,
            emojis_chance: builder.emojis_chance,
        }
    }
}

impl From<UwuBuilder> for Uwu {
    fn from(builder: UwuBuilder) -> Self {
        Self::from(&builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_builder() {
        let builder_ver = Uwu::builder()
            .lowercase()
            .expressions()
            .w_replace()
            .stutter(4)
            .emojis(1)
            .build();

        let manual_ver = Uwu {
            lowercase: true,
            expressions: true,
            w_replace: true,
            stutter: true,
            stutter_chance: 4,
            emojis: true,
            emojis_chance: 1,
        };

        assert_eq!(builder_ver, manual_ver);
    }
}
