local List = require 'pandoc.List'
function symbols(elem)
    if elem.text == "{{checkmark}}" then
        local x = {}
        if FORMAT:match 'latex' then
            table.insert(x, pandoc.RawInline('latex', '\\checkmark'))
        end
        return x
    else
        return elem
    end
end

return {symbols=symbols}
