// ...
Weapon createProceduralWeapon(){
    Sprite body* = new Sprite("body0001.png");
    Sprite scope* = new Sprite("scope0051.png");
    Sprite barrel* = new Sprite("barrel0045.png");
    Sprite ammo_mag* = new Sprite("mag0009.png");
    Weapon weapon* = Weapon.composeWeapon(body, scope, barrel, ammo_mag);
    weapon->setDamage(45);
    weapon->setRange(15);
    weapon->setSpread(23);
    return weapon;
}
// ...
