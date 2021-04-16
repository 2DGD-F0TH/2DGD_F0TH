local List = require 'pandoc.List'
function epigraphs(elem)
    if FORMAT:match 'latex' then
        -- Trivia box
        if elem.classes[1] == 'epigraph' then
           local author = elem.attributes["author"]
           local quote = pandoc.utils.stringify(elem.content)
           quote = quote:gsub("%%", "\\%%")
           return {
              pandoc.RawBlock("latex", "\\epigraph{" .. quote .. "}{\\textit{" .. author .. "}}"),
           }
        end
    else
        if elem.classes[1] == 'epigraph' then
            local author = elem.attributes["author"]
            if author ~=nil then
                table.insert(elem.content, pandoc.RawBlock("html", '<div class="epigraph-author">' .. author .. '</div>'))
            end
        end
        return elem
    end
end

return {epigraphs=epigraphs}
