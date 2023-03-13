// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T> {
    s: String,
    f: Box<dyn Fn(T) -> bool + 'a>,
}

impl<'a, T> Matcher<'a, T> {
    pub fn new<P, S>(_matcher: P, _subs: S) -> Matcher<'a, T>
    where
        P: Fn(T) -> bool + 'a,
        S: AsRef<str>,
    {
        Self {
            s: _subs.as_ref().to_string(),
            f: Box::new(_matcher),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<'a, T>(Vec<Matcher<'a, T>>);

impl<'a, T> Fizzy<'a, T>
where
    T: ToString + Clone + 'a,
{
    pub fn new() -> Self {
        Self(vec![])
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.0.push(matcher);
        self
    }

    fn subs(&self, i: &T) -> Vec<String> {
        (self.0)
            .iter()
            .filter_map(|ma| match (ma.f)(i.clone()) {
                true => Some(ma.s.to_owned()),
                false => None,
            })
            .collect()
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, _iter: I) -> impl Iterator<Item = String> + 'a
    where
        I: IntoIterator<Item = T> + 'a,
    {
        _iter.into_iter().map(move |ref i| {
            let subs = self.subs(i);
            match subs.is_empty() {
                true => i.to_string(),
                false => subs.join(""),
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: Clone + PartialEq + std::ops::Rem<T, Output = T> + ToString + 'a,
    u8: Into<T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
