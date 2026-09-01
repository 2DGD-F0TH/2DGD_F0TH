struct LazySingleton;

impl LazySingleton {
    fn new() -> Self {
        Self
    }

    pub fn instance() -> &'static Self {
        static INSTANCE: std::sync::LazyLock<LazySingleton> =
            std::sync::LazyLock::new(LazySingleton::new);

        &*INSTANCE
    }
}
