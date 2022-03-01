local function binarySearch(elements, element_to_find, begin_idx, end_idx)
    begin_idx = begin_idx or 1
    end_idx = end_idx or #elements
    -- We use // for an integer division
    local middle_index = ((end_idx - begin_idx) // 2) + begin_idx
    local middle_element = elements[middle_index]
    if (end_idx <= begin_idx and element_to_find ~= middle_element) then
        -- We found nothing, if we don't have this base case the algorithm will keep going forever
        return nil
    end
    if (element_to_find == middle_element) then
        return middle_index
    end
    if (element_to_find > middle_element) then
        return binarySearch(elements, element_to_find, middle_index + 1, end_idx)
    else
        return binarySearch(elements, element_to_find, begin_idx, middle_index - 1)
    end
end
