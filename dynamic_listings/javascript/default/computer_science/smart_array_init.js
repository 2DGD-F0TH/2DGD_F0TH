// ...
const tyler_array_len = 64;
let tyler = [];
for (let i = 0; i < tyler_array_len; i++) {
    tyler.push(CharacterExpression.neutral);
}
// Here we take care of the exceptions
tyler[33] = CharacterExpression.angry;
tyler[34] = CharacterExpression.sad;
// ... Next character ...
