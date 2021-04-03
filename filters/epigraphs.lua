local List = require 'pandoc.List'
function epigraphs(elem)
    if FORMAT:match 'latex' then
        -- Trivia box
        if elem.classes[1] == 'epigraph' then
           local author = elem.attributes["author"]
           return {
              pandoc.RawBlock("latex", "\\epigraph{" .. pandoc.utils.stringify(elem.content) .. "}{\\textit{" .. author .. "}}"),
           }
        end
    else
        if elem.classes[1] == 'epigraph' then
            local author = elem.attributes["author"]
            table.insert(elem.content, pandoc.RawBlock("html", '<div class="epigraph-author">' .. author .. '</div>'))
        end
        return elem
    end
end

return {epigraphs=epigraphs}
