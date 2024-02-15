function binarySearch(elements, element_to_find, begin, end){
    // We need some indexes to make this work
    if (begin === undefined){
        begin = 0;
    }
    if (end === undefined){
        end = elements.length - 1;
    }
    // Get the middle element
    let middle_element_index = Math.floor((end - begin) / 2) + begin;
    let middle_element = elements[middle_element_index];
    if (end <= begin && element_to_find != middle_element){
        // We found nothing, if we don't have this base case the algorithm will keep going forever
        return null;
    }
    if (element_to_find == middle_element){
        return middle_element_index;
    }
    if (element_to_find > middle_element){
        return binarySearch(elements, element_to_find, middle_element_index + 1, end);
    }else{
        return binarySearch(elements, element_to_find, begin, middle_element_index - 1);
    }
}
