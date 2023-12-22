#!/bin/sh

deploy(){
    # Get the version of the ebook from the latest git tag
    VERSION=$1
    # Declare an associative array
    declare -A CHANNELS

    # Declare the channels
    CHANNELS["pseudocode"]="output/pseudocode/"
    CHANNELS["python"]="output/python/"
    CHANNELS["cpp"]="output/cpp"
    CHANNELS["javascript"]="output/js/"
    CHANNELS["lua"]="output/lua/"
    CHANNELS["complete_collection"]="output/"

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
