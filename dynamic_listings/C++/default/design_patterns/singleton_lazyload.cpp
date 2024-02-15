class LazySingleton {

    private:
        static LazySingleton* instance = nullptr;
        LazySingleton() {}

    public:
        static Singleton getInstance() {
            // Multi-threading: manage race conditions
            // ----- Critical region start -----
            if (instance == nullptr) {
                instance = new Singleton();
            }
            // ----- Critical region end -----

            return *instance;
        }
};
