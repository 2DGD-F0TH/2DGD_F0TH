function createRandomizedWeapon()
    math.randomseed(os.time())
    local wp = new Weapon()
    -- Load some randomized components
    wp.loadRandomBodyFrom("weaponBodies/shotguns")
    wp.loadRandomScopeFrom("weaponScopes/shotguns")
    wp.loadRandomBarrelFrom("weaponBarrels/shotguns")
    wp.loadRandomAmmoMagazineFrom("weaponMagazines/shotguns")
    -- set weapon damage to a value between 35 and 50
    wp.setDamage(math.random(35, 50))
    -- set weapon range to a value between 13 and 18
    wp.setRange(math.random(13, 18))
    -- set weapon spread to a value between 20 and 30
    wp.setSpread(math.random(20, 30))
    return wp
end
