#!/bin/sh

deploy(){
    # Get the version of the ebook from the latest git tag
    VERSION=$1
    # Declare an associative array
    declare -A CHANNELS

    # Declare the channels
    CHANNELS["pseudocode"]="output/Pseudocode_Edition.pdf"
    CHANNELS["pseudocode-epub"]="output/Pseudocode_Edition.epub"
    CHANNELS["python"]="output/Python_Edition.pdf"
    CHANNELS["python-epub"]="output/Python_Edition.epub"
    CHANNELS["cpp"]="output/C++_Edition.pdf"
    CHANNELS["cpp-epub"]="output/C++_Edition.epub"
    CHANNELS["javascript"]="output/JS_Edition.pdf"
    CHANNELS["javascript-epub"]="output/JS_Edition.epub"
    CHANNELS["lua"]="output/Lua_Edition.pdf"
    CHANNELS["lua-epub"]="output/Lua_Edition.epub"

    # Do the push thingy
    for channel in ${!CHANNELS[@]}; do
        FILENAME=${CHANNELS[${channel}]}
        echo "$(tput setaf 5)Pushing ${FILENAME} to channel ${channel}$(tput setaf 7) (version ${VERSION})"
        butler push --if-changed --context-timeout=60 ${FILENAME} therealpenaz91/2dgd-f0th:${channel} --userversion ${VERSION}
    done
}

# Get confirmation first!

VERSION=$(git describe --tags)

echo "Deploying version ${VERSION}"

read -p "Would you like to continue with the deploy? (y/n) " choice

case $choice in
    y|Y ) deploy $VERSION;;
    n|N ) exit 1;;
    * ) echo "Invalid choice, assuming 'no'."; exit 1;;
esac
