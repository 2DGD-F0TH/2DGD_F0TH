local boxes = require "filters/boxes"
local placeholder = require "filters/placeholder"
local pagebreak = require "filters/pagebreak"
local metavars = require "filters/metavars"

return {
        -- Meta
        {Meta = metavars.get_vars},
        -- Inlines
        {Str = metavars.replace},
        {Str = placeholder.placeholder},
        {Str = pagebreak.pagebreak},
        -- Blocks
        {Div = boxes.boxes}
}
