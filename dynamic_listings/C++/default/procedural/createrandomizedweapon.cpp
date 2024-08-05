#include <vector>
#include <filesystem>
//...
void get_directory_listing(std::vector<std::string>* vect, const std::string& path){
    /* Fills the vector "vect" by side effect */
    for (auto p: std::filesystem::directory_iterator(path)) {
        vect->push_back(p.path());
    }
    return;
}
//...
template <typename T>
T get_random(const std::vector<T> &vect){
    // Gets a random item from a Vector
    auto it = vect.begin();
    // C++ rand() will do for this (remember to seed the randomizer with srand())
    int random = rand() % vect.size();
    // Let's return the item via pointer arithmetics
    return *(it + random);
}
// ...
Weapon createRandomizedWeapon(){
    // Get the directory contents for each component
    std::vector<std::string> bodies_dir* = new std::vector<std::string>();
    std::vector<std::string> scopes_dir* = new std::vector<std::string>();
    std::vector<std::string> barrels_dir* = new std::vector<std::string>();
    std::vector<std::string> mags_dir* = new std::vector<std::string>();
    get_directory_listing(bodies_dir, "weaponBodies/shotguns");
    get_directory_listing(scopes_dir, "weaponScopes/shotguns");
    get_directory_listing(barrels_dir, "weaponBarrels/shotguns");
    get_directory_listing(mags_dir, "weaponMagazines/shotguns");
    // Get a random item for each component list
    Sprite body* = new Sprite(get_random<std::string>(*bodies_dir));
    Sprite scope* = new Sprite(get_random<std::string>(*scopes_dir));
    Sprite barrel* = new Sprite(get_random<std::string>(*barrels_dir));
    Sprite ammo_mag* = new Sprite(get_random<std::string>(*mag_dir));
    // Put the weapon together
    Weapon weapon* = Weapon.composeWeapon(body, scope, barrel, ammo_mag);
    // Set the randomly generated properties
    // To have a number between min and max the formula is rand()%(max-min + 1) + min;
    // Set weapon damage to a value between 35 and 50
    weapon->setDamage(rand() % (16) + 35);
    // Set weapon range to a value between 13 and 18
    weapon->setRange(rand() % (6) + 18);
    // Set weapon spread to a value between 20 and 30
    weapon->setSpread(rand() % (11) + 20);
    return weapon;
}
