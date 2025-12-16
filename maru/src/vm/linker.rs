


pub enum LinkerEntry<T> {
    Entry(T),
    Hole(Symbol),
}