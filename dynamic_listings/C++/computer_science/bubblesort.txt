void bubbleSort(int[] A){
    n = A.size();
    // Traverse Through all Elements
    for (i = 0; i < n; ++i) {
        // Last i elements are in place due to the nature of the sort
        for (j = 0; j < n - i - 1; ++j) {
            // Swap if the element found is greater than the next element
            if (A[j] > A[j+1]){
                int tmp = A[j];
                A[j] = A[j+1];
                A[j+1] = tmp;
            }
        }
    }
}
