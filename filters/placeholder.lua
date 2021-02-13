local List = require 'pandoc.List'
function placeholder(elem)
    if elem.text == "{{placeholder}}" then
        return pandoc.Strong {pandoc.Emph {pandoc.Str "[This section is a work in progress and it will be completed as soon as possible]"}}
    else
        return elem
    end
end

return {placeholder=placeholder}
