local List = require 'pandoc.List'
function symbols(elem)
    if FORMAT == 'latex' then
        if elem.text == "{{freeprod}}" then
            return pandoc.RawInline("latex", "\\freeprod")
        end
        if elem.text == "{{paidprod}}" then
            return pandoc.RawInline("latex", "\\paidprod")
        end
        if elem.text == "{{donprod}}" then
            return pandoc.RawInline("latex", "\\donprod")
        end
        return elem
    else
        if elem.text == "{{freeprod}}" then
            return pandoc.Span(pandoc.Strong("[F]"), {["style"] = "color: green;"})
        end
        if elem.text == "{{paidprod}}" then
            return pandoc.Span(pandoc.Strong("[P]"), {["style"] = "color: red;"})
        end
        if elem.text == "{{donprod}}" then
            return pandoc.Span(pandoc.Strong("[D]"), {["style"] = "color: orange;"})
        end
        return elem
    end
end

return {symbols=symbols}
