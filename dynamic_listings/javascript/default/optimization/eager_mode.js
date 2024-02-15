class EagerObject{
    constructor(numbers){
        this.halved_numbers = [];
        // Prepares the halved numbers list
        for (number in numbers){
            this.halved_numbers.push(number / 2);
        }
    }

    getObject(index){
        // Returns the pre-calculated object at the requested index
        return halved_numbers[index];
    }
}
