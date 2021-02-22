local boxes = require "filters/boxes"
local placeholder = require "filters/placeholder"
local pagebreak = require "filters/pagebreak"
local symbols = require "filters/symbols"

return {
        -- Inlines
        {Str = placeholder.placeholder},
        {Str = pagebreak.pagebreak},
        {Str = symbols.symbols},
        -- Blocks
        {Div = boxes.boxes}
        -- Meta
}
