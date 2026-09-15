pub struct ShadowBorrow<'a, T> {
    value: *mut T,
    _marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> ShadowBorrow<'a, T> {
    pub(crate) fn new(val: &'a mut T) -> (Self, &'a mut T) {
        let ptr = val as *mut T;
        let shadow = ShadowBorrow {
            value: ptr,
            _marker: std::marker::PhantomData,
        };
        // SAFETY: The pointer was just constructed from a valid reference and is thus valid to dereference.
        (shadow, unsafe { &mut *ptr })
    }

    pub(crate) fn call<B>(&self, data: &mut B, func: fn(NestedBorrow<'_, '_, B, T>)) {
        func(NestedBorrow {
            borrow: data,
            val: self.value,
            _marker: std::marker::PhantomData,
        });
    }

    /// This version of call doesn't enforce that you neither pass a reference to T to nor return one; You have to be carful yourself. Otherwise it is immediately UB, regardless of if you use the reference or not.
    pub(crate) unsafe fn call_unsafe<B, F: FnOnce(NestedBorrow<'_, '_, B, T>) -> R, R>(
        &self,
        data: &mut B,
        func: F,
    ) -> R {
        func(NestedBorrow {
            borrow: data,
            val: self.value,
            _marker: std::marker::PhantomData,
        })
    }
}

/// The goal of this struct is be able to pass both a reference to context and a reference to a specific object within the context to another function. To make this safe, the reference to the context is only available after the reference to the inner object is no longer accessible.
pub struct NestedBorrow<'a, 'b, B, T> {
    borrow: &'b mut B,
    val: *mut T,
    _marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, 'b, B, T> NestedBorrow<'a, 'b, B, T> {
    pub(crate) fn object(&mut self) -> &mut B {
        self.borrow
    }

    pub(crate) fn call(&mut self, func: fn(NestedBorrow<'a, 'b, B, T>)) {
        func(NestedBorrow {
            borrow: unsafe { &mut *(self.borrow as *mut B) },
            val: self.val,
            _marker: self._marker,
        });
    }

    pub(crate) fn finish(self) -> &'a mut T {
        // SAFETY: Pointer obtained from ShandowBorrow is safe to dereference and has a lifetime of 'a
        unsafe { &mut *self.val }
    }
}
