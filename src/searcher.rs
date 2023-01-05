mod default;
mod div8;
mod fast;

pub use self::default::DefaultSearcher;
pub use self::div8::Div8Searcher;
pub use self::fast::FastSearcher;

pub(crate) fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
