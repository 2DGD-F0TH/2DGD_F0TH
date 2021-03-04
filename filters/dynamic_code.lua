local List = require 'pandoc.List'
local metavars = require 'filters/metavars'

function dynamic_codeblock(blk)
    if blk.attributes.src then
        local content = ""
        local language = metavars.find_var("proglang")
        if language == nil or language == "" then
            language = "pseudocode"
        end
        local fullpath = "dynamic_listings/" .. language .. "/" .. blk.attributes.src .. ".txt"
        local file_handler = io.open(fullpath)
        if not file_handler then
            io.stderr:write("Cannot open file " .. fullpath .. " | Skipping\n")
        else
            for line in file_handler:lines ("L")
            do
                content = content .. line
            end
            file_handler:close()
        end
        blk.attributes.src = nil
        -- Set language
        blk.attributes.language = language
        code = pandoc.CodeBlock(content, blk.attr)
        code.classes = {language}
        -- TODO: Include caption in epub output
        return code
    end
end

return {dynamic_codeblock=dynamic_codeblock}
