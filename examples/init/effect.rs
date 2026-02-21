use effing_mad::effects;

effects! {
    pub Placing<T, R> {
        pub fn begin_init() -> Uninit<T>;
        pub fn finish_init(pub init: Init<T>) -> R;
    }
}

pub struct Uninit<T>(*mut T);

// Workaround for effing_mad not having single-use effects
impl<T> Clone for Uninit<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Uninit<T> {}

impl<T> Uninit<T> {
    pub unsafe fn finish_unchecked(self) -> Init<T> {
        Init(self.0)
    }

    pub fn as_ptr(&self) -> *mut T {
        self.0
    }

    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self(ptr)
    }
}

pub struct Init<T>(*mut T);

impl<T> Init<T> {
    pub fn as_raw(&self) -> *mut T {
        self.0
    }
}
