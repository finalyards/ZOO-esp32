# Stable of nightly?

## `type_alias_impl_trait`

Dario Nieuwenhuis gives a good summary on why nightly is (currently) a good thing for Embassy in [this video](https://youtu.be/H7NtzyP9q8E?si=3CtwuAAM6m0Vj69M&t=860). 

TL;DW: The `Task` system needs type aliasing for `impl Future` - and that's a nightly feature. There seems to be a "fallback" for stable, but... this author is convinced to keep using nightly, for now! :)

[Track here](https://github.com/rust-lang/rust/issues/63063).
<!-- AI generated track marks here -->
