local boxes = require "filters/boxes"
local placeholder = require "filters/placeholder"
local pagebreak = require "filters/pagebreak"

return {
        {Div = boxes.boxes},
        {Str = placeholder.placeholder},
        {Str = pagebreak.pagebreak}
}
