int binarySearch(int[] lst, int item){
    int first = 0;
    int last = lst.size() - 1;
    bool found = false;
    while(first <= last && !found){
        // Find the middle element
        int midpoint = (int) (first + last) / 2;
        if (lst[midpoint] == item){
            // We found it!
            return midpoint;
        }else{
            if (item < lst[midpoint]){
                // Continue on the "first half"
                last = midpoint - 1;
            }else{
                // Continue on the "second half"
                first = midpoint + 1;
            }
        }
    }
    if (!found){
        // We return -1 to tell "not found"
        return -1;
    }
}
