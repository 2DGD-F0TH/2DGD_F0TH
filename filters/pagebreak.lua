local List = require 'pandoc.List'
function pagebreak(elem)
    if elem.text == "{{pagebreak}}" then
        print("Found")
        local x = {}
        if FORMAT:match 'latex' then
            print("Run")
            table.insert(x, pandoc.RawInline('latex', '\\null\\clearpage'))
        end
        return x
    else
        return elem
    end
end

return {pagebreak=pagebreak}
