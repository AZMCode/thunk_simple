//! Simpler alternative to the `thunk` crate, containing only one `Thunk` type, and conversion between Iterators and Thunks

/// The `Thunk<T>` type.
/// This type is not evaluated until the `unwrap` method is called.
/// The inner value cannot be referenced until it's unwrapped.
/// 
/// ## Panics
/// If the Thunk was created `from` an Iterator, the Thunk may panic if the Iterator yields less than one element.
/// Otherwise only panics when any function involved in calculating the value panics.
pub struct Thunk<'f,T>(Box<dyn (FnOnce()-> T) + 'f>) where T: 'f;

impl<'f,T> Thunk<'f,T> where T:'f {

    /// Create a new `Thunk<T>` from a closure or function returning `T`
    pub fn new<NT,NF>(body: NF) -> Thunk<'f,NT> where
        NF: (FnOnce() -> NT) + 'f
    {
        Thunk(Box::new(body))
    }

    /// Create a new `Thunk<T>` from an already-calculated value T
    pub fn new_const<NT: 'f>(nt: NT) -> Thunk<'f,NT> {
        Self::new(|| nt)
    }

    /// Map a `Thunk<T>` into a `Thunk<U>`.
    /// This method supplies the previous thunk value unevaluated as an argument.
    pub fn map_lazy<NT,NF>(self,body: NF) -> Thunk<'f,NT> where
        NF: (FnOnce(Thunk<'f,T>) -> NT) + 'f
    {
        Thunk::<'f,NT>::new(|| (body)(self))
    }

    /// Map a `Thunk<T>` into a `Thunk<U>`.
    /// This method provides an already-evaluated T to the mapping function
    pub fn map<NT,NF>(self,body: NF) -> Thunk<'f,NT> where
        NF: (FnOnce(T) -> NT) + 'f
    {
        Thunk::<'f,NT>::new(|| (body)((self.0)()))
    }

    /// Unwrap, and therefore calculate, the value `T` contained inside this thunk
    pub fn unwrap(self) -> T {
        (self.0)()
    }

}

impl<'f,T,I> From<I> for Thunk<'f,T> where
    T: 'f,
    I: Iterator<Item = T> + 'f
{
    fn from(mut iter: I) -> Self {
        Self::new(move || iter.next().unwrap())
    }
}

impl<'f,T> IntoIterator for Thunk<'f,T> where
    T: 'f
{
    type IntoIter = IntoIter<'f,T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(Some(self))
    }
}

/// Simple IntoIter Iterator, that yields the value of the Thunk then None
pub struct IntoIter<'f,T>(Option<Thunk<'f,T>>);

impl<'f,T> Iterator for IntoIter<'f,T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(thunk) = std::mem::take(&mut self.0) {
            Some(thunk.unwrap())
        } else {
            None
        }
    }
}