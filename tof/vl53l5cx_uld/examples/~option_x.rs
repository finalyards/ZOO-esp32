/*
* Adding '.for_each()' to 'Option'.
*
* The author doesn't like the 'if let' syntax, at all. Alternatives:
*   - 'opt.into_iter().for_each( |x| ... );     // works, clunky
*   - 'for x in opt ...'                        // nicer (though not postfix), but requires an '#[allow(for_loops_over_fallibles)]'
*/

pub trait OptionExt<T> where Self: IntoIterator {
    fn for_each(&self, f: impl FnMut(Self::Item)) {
        self.into_iter().for_each(f)
    }
}

impl<T> OptionExt<T> for Option<T> {}
