macro_rules! offset_of {
    ($ty:ty, $field:ident $(,)?) => ({
        let null = 0usize as *const $ty;
        std::mem::transmute::<_, usize>(&(*null).$field as *const _)
    });
}

pub(crate) use offset_of;