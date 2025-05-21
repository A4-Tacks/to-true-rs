#![no_std]
#![doc = include_str!("../README.md")]

#[doc = include_str!("../README.md")]
pub trait ToTrue: Sized {
    /// Run `f` when `*self == false`, then assign `*self` to `true`
    fn to_true<F, R>(&mut self, f: F) -> Option<R>
    where F: FnOnce() -> R;

    /// Run `f` when `*self == true`, then assign `*self` to `false`
    fn to_false<F, R>(&mut self, f: F) -> Option<R>
    where F: FnOnce() -> R;
}
impl ToTrue for bool {
    fn to_true<F, R>(&mut self, f: F) -> Option<R>
    where F: FnOnce() -> R,
    {
        if *self {
            None
        } else {
            *self = true;
            Some(f())
        }
    }

    fn to_false<F, R>(&mut self, f: F) -> Option<R>
    where F: FnOnce() -> R,
    {
        if *self {
            *self = false;
            Some(f())
        } else {
            None
        }
    }
}
