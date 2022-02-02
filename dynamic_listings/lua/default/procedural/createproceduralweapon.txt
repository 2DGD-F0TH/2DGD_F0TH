function createProceduralWeapon()
    local wp = Weapon:new()
    -- Load some pre-defined components
    wp:loadBody("body0001.png")
    wp:loadScope("scope0051.png")
    wp:loadBarrel("barrel0045.png")
    wp:loadAmmoMagazine("mag0009.png")
    -- set weapon damage to 45
    wp:setDamage(45)
    -- set weapon range to 15
    wp:setRange(15)
    -- set weapon spread to 23
    wp:setSpread(23)
    return wp
end
