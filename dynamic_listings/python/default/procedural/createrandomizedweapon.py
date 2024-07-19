from random import choice as random_choice
from random import randint
from os import listdir


def create_randomized_weapon() -> Weapon:
    # Start by getting the directory contets for each component
    bodies: list[str] = listdir("weaponBodies/shotguns")
    scopes: list[str] = listdir("weaponScopes/shotguns")
    barrels: list[str] = listdir("weaponBarrels/shotguns")
    mags: list[str] = listdir("weaponMagazines/shotguns")
    # Choose a random piece for each component
    body: Sprite = Sprite(random_choice(bodies))
    scope: Sprite = Sprite(random_choice(scopes))
    barrel: Sprite = Sprite(random_choice(barrels))
    ammo_mag: Sprite = Sprite(random_choice(mags))
    weapon: Weapon = compose_weapon(body, scope, barrel, ammo_mag)
    # Set weapon damage to a value between 35 and 50
    weapon.set_damage(randint(35, 50))
    # Set weapon range to a value between 13 and 18
    weapon.set_range(randint(13,28))
    # Set weapon spread to a value between 20 and 30
    weapon.set_spread(randint(20,30))
    return weapon

