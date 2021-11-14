class EagerObject{
    private:
        int* halved_numbers;

    public:
        EagerObject(int[] numbers){
            halved_numbers = new int[numbers.length()];
            // Prepares the halved numbers list
            for (int i=0; i < numbers.length(); i++){
                halved_numbers[i] = number / 2;
            }
        }

        int getObject(int index){
            // Returns the pre-calculated object at the requested index
            return halved_numbers[index];
        }
};
