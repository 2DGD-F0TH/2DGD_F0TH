def create_procedural_weapon():
    body = Sprite("body0001.png")
    scope = Sprite("scope0051.png")
    barrel = Sprite("barrel0045.png")
    ammo_mag = Sprite("mag0009.png")
    weapon = compose_weapon(body, scope, barrel, ammo_mag)
    weapon.set_damage(45)
    weapon.set_range(15)
    weapon.set_spread(23)
    return weapon
