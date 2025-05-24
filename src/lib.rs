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

/// ```
/// # use to_true::InTrue;
/// let mut state = false;
/// let mut n = 0;
///
/// assert_eq!(state.in_true(|| n += 1), None);
/// assert_eq!((n, state), (0, true));
///
/// assert_eq!(state.in_true(|| n += 1), Some(()));
/// assert_eq!((n, state), (1, true));
///
/// assert_eq!(state.in_false(|| n += 1), None);
/// assert_eq!((n, state), (1, false));
///
/// assert_eq!(state.in_false(|| n += 1), Some(()));
/// assert_eq!((n, state), (2, false));
/// ```
pub trait InTrue: Sized {
    /// Run `f` when `*self == true`, then assign `*self` to `true`
    fn in_true<F, R>(&mut self, f: F) -> Option<R>
    where F: FnOnce() -> R;

    /// Run `f` when `*self == false`, then assign `*self` to `false`
    fn in_false<F, R>(&mut self, f: F) -> Option<R>
    where F: FnOnce() -> R;
}
impl InTrue for bool {
    fn in_true<F, R>(&mut self, f: F) -> Option<R>
    where F: FnOnce() -> R,
    {
        let old = *self;
        *self = true;
        old.then(f)
    }

    fn in_false<F, R>(&mut self, f: F) -> Option<R>
    where F: FnOnce() -> R,
    {
        let old = *self;
        *self = false;
        (!old).then(f)
    }
}
