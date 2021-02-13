local List = require 'pandoc.List'
function boxes(elem)
    if FORMAT:match 'latex' then
        -- Trivia box
        if elem.classes[1] == 'trivia' then
            table.insert(elem.content, 1, pandoc.RawBlock('latex', '\\begin{trivia}'))
            table.insert(elem.content, pandoc.RawBlock('latex', '\\end{trivia}'))
        end
        -- Pitfall Box
        if elem.classes[1] == 'pitfall' then
            table.insert(elem.content, 1, pandoc.RawBlock('latex', '\\begin{pitfall}'))
            table.insert(elem.content, pandoc.RawBlock('latex', '\\end{pitfall}'))
        end
        -- Tip Box
        if elem.classes[1] == 'tip' then
            table.insert(elem.content, 1, pandoc.RawBlock('latex', '\\begin{tip}'))
            table.insert(elem.content, pandoc.RawBlock('latex', '\\end{tip}'))
        end
        return elem.content
    else
        return elem
    end
end

return {boxes=boxes}
