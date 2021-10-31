class LazyObject{
    private:
        int* numbers_reference;

    public:
        LazyObject(int[] numbers){
            // Saves the original list (possibly as a reference)
            numbers_reference = numbers;
        }

        int getObject(int index){
            // Calculates the halved number on-demand
            return numbers_reference[index] / 2;
        }
};
