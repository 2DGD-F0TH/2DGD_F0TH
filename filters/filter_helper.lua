local boxes = require "filters/boxes"
local placeholder = require "filters/placeholder"
return {{
        Div = boxes.boxes,
        Str = placeholder.placeholder
}}
