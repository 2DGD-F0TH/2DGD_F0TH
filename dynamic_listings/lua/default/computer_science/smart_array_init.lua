-- ...
local tyler_array_len = 64
local tyler = {}
for _ = 1, #tyler_array_len do
    table.insert(tyler, CharacterExpression.neutral)
end
-- Here we take care of the exceptions
tyler[34] = CharacterExpression.angry
tyler[35] = CharacterExpression.sad
-- ... Next character ...
