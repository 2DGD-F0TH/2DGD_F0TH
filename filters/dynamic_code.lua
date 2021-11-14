local List = require 'pandoc.List'
local metavars = require 'filters/metavars'

function dynamic_codeblock(blk)
    if blk.attributes.src then
        local content = ""
        local language = metavars.find_var("proglang")
        if language == nil or language == "" then
            language = "pseudocode"
        end
        local extension = metavars.find_var("lang_extension")
        if extension == nil or extension == "" then
            extension = "default"
        end
        local fullpath = "dynamic_listings/" .. language .. "/" .. extension .. "/" .. blk.attributes.src .. ".txt"
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
        if language == "pseudocode" then
            -- Workaround to allow pseudocode to be enclosed in frames
            table.insert(code.classes, "sourceCode")
        end
        if FORMAT ~= "latex" then
            if blk.attributes.caption then
                local caption = pandoc.Div(pandoc.Para(pandoc.Str("Code: " .. blk.attributes.caption)))
                caption.classes = {"code-caption"}
                return {
                    code,
                    caption
                }
            end
        end
        return code
    end
end

return {dynamic_codeblock=dynamic_codeblock}
