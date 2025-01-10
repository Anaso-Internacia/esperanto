/// Extension to [`char`] to include Esperanto-related methods.
pub trait EoCharExt {
    /// Returns `true` if this `char` is a valid Esperanto character.
    ///
    /// True for `A-Z` and `a-z`, including diacritics (`Ĉ`, `Ĵ`), excluding non-eo (`X`, `Y`).
    #[must_use]
    fn is_esperantic(&self) -> bool;

    /// Returns `true` if this `char` satisfies either [`is_esperantic()`] or [`is_ascii_digit()`].
    #[must_use]
    fn is_esperantonumeric(&self) -> bool;
}

impl EoCharExt for char {
    #[inline]
    fn is_esperantic(&self) -> bool {
        matches!(*self,
            'A'..='P' | 'R'..='V' | 'Z' | 'Ĉ' | 'Ĝ' | 'Ĥ' | 'Ĵ' | 'Ŝ' | 'Ŭ' |
            'a'..='p' | 'r'..='v' | 'z' | 'ĉ' | 'ĝ' | 'ĥ' | 'ĵ' | 'ŝ' | 'ŭ'
        )
    }

    #[inline]
    fn is_esperantonumeric(&self) -> bool {
        self.is_esperantic() || self.is_ascii_digit()
    }
}
