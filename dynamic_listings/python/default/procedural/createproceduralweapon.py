def create_procedural_weapon() -> Weapon:
    body: Sprite = Sprite("body0001.png")
    scope: Sprite = Sprite("scope0051.png")
    barrel: Sprite = Sprite("barrel0045.png")
    ammo_mag: Sprite = Sprite("mag0009.png")
    weapon: Weapon = compose_weapon(body, scope, barrel, ammo_mag)
    weapon.set_damage(45)
    weapon.set_range(15)
    weapon.set_spread(23)
    return weapon
