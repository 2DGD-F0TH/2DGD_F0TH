# #![allow(dead_code, unused)]
#
# enum CharacterExpression {
#     Neutral,
#     Angry,
#     Sad,
# }
# fn main() {
// ...
let tyler_array_len = 64;
let mut tyler = Vec::new();
for _ in 0..64 {
    tyler.push(CharacterExpression::Neutral);
}
// Here we take care of the exceptions
tyler[33] = CharacterExpression::Angry;
tyler[34] = CharacterExpression::Sad;
// ... Next character ...
# }
