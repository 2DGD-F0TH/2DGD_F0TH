# #![allow(dead_code)]
# struct Sprite;
# impl Sprite {
#     fn from<P: AsRef<std::path::Path>>(_: P) -> Self { todo!() }
# }
# struct Weapon;
# impl Weapon {
#     fn compose(_: Sprite, _: Sprite, _: Sprite, _: Sprite) -> Self { todo!() }
#     fn set_damage(&mut self, _: u32) {}
#     fn set_range(&mut self, _: u32) {}
#     fn set_spread(&mut self, _: u32) {}
# }
// ...
fn create_procedural_weapon() -> Weapon {
    let body = Sprite::from("body0001.png");
    let scope = Sprite::from("scope0051.png");
    let barrel = Sprite::from("barrel0045.png");
    let ammo_mag = Sprite::from("mag0009.png");
    let mut weapon = Weapon::compose(body, scope, barrel, ammo_mag);
    weapon.set_damage(45);
    weapon.set_range(15);
    weapon.set_spread(23);
    weapon
}
// ...
