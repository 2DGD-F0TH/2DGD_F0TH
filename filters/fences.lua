local List = require 'pandoc.List'
function fences(elem)
    if FORMAT:match 'latex' then
        -- Centering
        if elem.classes[1] == 'centering' then
            table.insert(elem.content, 1, pandoc.RawBlock('latex', '\\begin{center}'))
            table.insert(elem.content, pandoc.RawBlock('latex', '\\end{center}'))
        end
        return elem.content
    else
        return elem
    end
end

return {fences=fences}
