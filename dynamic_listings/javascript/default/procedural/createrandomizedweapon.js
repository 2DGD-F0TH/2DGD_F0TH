function createRandomizedWeapon(){
    // The "class" and "new" keywords are supported in ES6/ECMAScript 2015
    let wp = new Weapon();
    // Load some randomized components
    wp.loadRandomBodyFrom("weaponBodies/shotguns");
    wp.loadRandomScopeFrom("weaponScopes/shotguns");
    wp.loadRandomBarrelFrom("weaponBarrels/shotguns");
    wp.loadRandomAmmoMagazineFrom("weaponMagazines/shotguns");
    // set weapon damage to a value between 35 and 50
    wp.damage = Math.random() * 16 + 35;
    // set weapon range to a value between 13 and 18
    wp.range = Math.random() * 5 + 13;
    // set weapon spread to a value between 20 and 30
    wp.spread = Math.random() * 10 + 20;
    return wp;
}
