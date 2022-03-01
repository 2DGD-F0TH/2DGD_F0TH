local function bubbleSort(A)
    local swapped = false
    repeat
        swapped = false
        for i = 1, #A do
            if (A[i-1] > A[i]) then
                local tmp = A[i-1]
                A[i-1] = A[i]
                A[i] = tmp
                swapped = true
            end
        end
    until (not swapped)
end
