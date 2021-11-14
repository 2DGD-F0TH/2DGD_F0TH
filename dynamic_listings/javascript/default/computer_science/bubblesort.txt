function bubbleSort(A){
    let n = length(A);
    let swapped = false;
    do{
        swapped = false;
        for (let i = 1; i < n; i++){
            if (A[i-1] > A[i]){
                // Swap the items
                let tmp = A[i-1];
                A[i-1] = A[i];
                A[i] = tmp;
                swapped = true;
            }
        }
    }while(swapped);
}
