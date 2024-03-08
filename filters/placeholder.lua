local List = require 'pandoc.List'
function placeholder(elem)
    if elem.text == "{{placeholder}}" then
        return pandoc.Strong {pandoc.Emph {pandoc.Str "[This section is a work in progress and it will be completed as soon as possible]"}}
    elseif elem.text == "{{extend}}" then
        return pandoc.Strong {pandoc.Emph {pandoc.Str "[Do you know more about this matter? You can contribute to this book!]"}}
    else
        return elem
    end
end

return {placeholder=placeholder}
