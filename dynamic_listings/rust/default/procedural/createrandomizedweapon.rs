# //! ```cargo
# //! [dependencies]
# //! rand = "0.10"
# //! ```
#
# #![allow(dead_code, unused)]
#
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
fn directory_listing<P: AsRef<std::path::Path>>(
    path: P,
) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut vec = Vec::new();

    for entry in std::fs::read_dir(path)? {
        vec.push(entry?.path());
    }

    Ok(vec)
}

fn random<T>(vect: &[T]) -> &T {
    // Gets a random item from a Vector
    let x = rand::random_range(0..vect.len());
    &vect[x]
}

impl Weapon {
    pub fn create_randomized() -> Self {
        // Get the directory contents for each component
        let bodies_dir = directory_listing("weaponBodies/shotguns").unwrap();
        let scopes_dir = directory_listing("weaponScopes/shotguns").unwrap();
        let barrels_dir = directory_listing("weaponBarrels/shotguns").unwrap();
        let mags_dir = directory_listing("weaponMagazines/shotguns").unwrap();
        // Get a random item for each component list
        let body = Sprite::from(random(&bodies_dir));
        let scope = Sprite::from(random(&scopes_dir));
        let barrel = Sprite::from(random(&barrels_dir));
        let ammo_mag = Sprite::from(random(&mags_dir));
        // Put the weapon together
        let mut weapon = Weapon::compose(body, scope, barrel, ammo_mag);
        // Set the randomly generated properties
        // To have a number between min and max the formula is rand()%(max-min + 1) + min;
        // Set weapon damage to a value between 35 and 50
        weapon.set_damage(rand::random_range(35..=50));
        // Set weapon range to a value between 13 and 18
        weapon.set_range(rand::random_range(13..=18));
        // Set weapon spread to a value between 20 and 30
        weapon.set_spread(rand::random_range(20..=30));
        weapon
    }
}
