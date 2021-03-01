local boxes = require "filters/boxes"
local placeholder = require "filters/placeholder"
local pagebreak = require "filters/pagebreak"
local metavars = require "filters/metavars"
local dcb = require "filters/dynamic_code"

return {
        -- Meta
        {Meta = metavars.get_vars},
        -- Inlines
        {Str = metavars.replace},
        {Str = placeholder.placeholder},
        {Str = pagebreak.pagebreak},
        -- Blocks
        {Div = boxes.boxes},
        {CodeBlock = dcb.dynamic_codeblock}
}
