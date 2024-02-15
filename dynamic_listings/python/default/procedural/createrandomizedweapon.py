from random import choice as random_choice
from random import randint
from os import listdir


def createRandomizedWeapon():
    # Start by getting the directory contets for each component
    bodies = listdir("weaponBodies/shotguns")
    scopes = listdir("weaponScopes/shotguns")
    barrels = listdir("weaponBarrels/shotguns")
    mags = listdir("weaponMagazines/shotguns")
    # Choose a random piece for each component
    body = Sprite(random_choice(bodies))
    scope = Sprite(random_choice(scopes))
    barrel = Sprite(random_choice(barrels))
    ammo_mag = Sprite(random_choice(mags))
    weapon = composeWeapon(body, scope, barrel, ammo_mag)
    # Set weapon damage to a value between 35 and 50
    weapon.setDamage(randint(35, 50))
    # Set weapon range to a value between 13 and 18
    weapon.setRange(randint(13,28))
    # Set weapon spread to a value between 20 and 30
    weapon.setSpread(randint(20,30))
    return weapon
