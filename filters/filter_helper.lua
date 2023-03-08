local boxes = require "filters/boxes"
local placeholder = require "filters/placeholder"
local pagebreak = require "filters/pagebreak"
local metavars = require "filters/metavars"
local dcb = require "filters/dynamic_code"
local epigraphs = require "filters/epigraphs"
local symbols = require "filters/symbols"

return {
        -- Meta
        {Meta = metavars.get_vars},
        -- Inlines
        {Str = metavars.replace},
        {Str = placeholder.placeholder},
        {Str = pagebreak.pagebreak},
        {Str = symbols.symbols},
        -- Blocks
        {Div = epigraphs.epigraphs},
        {Div = boxes.boxes},
        {CodeBlock = dcb.dynamic_codeblock}
}
