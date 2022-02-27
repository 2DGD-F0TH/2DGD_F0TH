local A = {...}
---------------------------------------
local n = #A
local swapped = false
repeat
    swapped = false
    for i = 1, n-1 do
        if (A[i-1] > A[i]) then
            local tmp = A[i-1]
            A[i-1] = A[i]
            A[i] = tmp
            swapped = true
        end
    end
until (not swapped)

---------------------------------------

for i = 1, #A do
    print(A[i])
end
