class LazyObject{
    constructor(numbers){
        this.numbers_reference = numbers;
    }

    getObject(index){
        // Calculates the halved number on-demand
        return this.numbers_reference[index] / 2;
    }
}
