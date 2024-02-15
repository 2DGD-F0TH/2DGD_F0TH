def createProceduralWeapon():
    body = Sprite("body0001.png")
    scope = Sprite("scope0051.png")
    barrel = Sprite("barrel0045.png")
    ammo_mag = Sprite("mag0009.png")
    weapon = composeWeapon(body, scope, barrel, ammo_mag)
    weapon.setDamage(45)
    weapon.setRange(15)
    weapon.setSpread(23)
    return weapon
