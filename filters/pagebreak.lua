local List = require 'pandoc.List'
function pagebreak(elem)
    if elem.text == "{{pagebreak}}" then
        local x = {}
        if FORMAT:match 'latex' then
            table.insert(x, pandoc.RawInline('latex', '\\null\\clearpage'))
        end
        return x
    else
        return elem
    end
end

return {pagebreak=pagebreak}
