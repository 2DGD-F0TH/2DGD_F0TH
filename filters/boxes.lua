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
        -- Note Box
        if elem.classes[1] == 'note' then
            table.insert(elem.content, 1, pandoc.RawBlock('latex', '\\begin{note}'))
            table.insert(elem.content, pandoc.RawBlock('latex', '\\end{note}'))
        end
        return elem.content
    else
        if elem.classes[1] == 'trivia' then
            table.insert(elem.content, 1, pandoc.RawBlock('html', '<div class="box-title">Random Trivia!</div>'))
        end
        if elem.classes[1] == 'tip' then
            table.insert(elem.content, 1, pandoc.RawBlock('html', '<div class="box-title">Tip!</div>'))
        end
        if elem.classes[1] == 'pitfall' then
            table.insert(elem.content, 1, pandoc.RawBlock('html', '<div class="box-title">Pitfall Warning!</div>'))
        end
        if elem.classes[1] == 'note' then
            table.insert(elem.content, 1, pandoc.RawBlock('html', '<div class="box-title">Note!</div>'))
        end
        return elem
    end
end

return {boxes=boxes}
