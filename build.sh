#!/usr/bin/env sh

DIALOG_CMD=dialog

# Check if dialog is installed

if ! command -v ${DIALOG_CMD} &> /dev/null
then
    echo "Dialog is not installed in your system, cannot use interactive build";
    exit 1;
fi

# Dialog is installed, open the interactive screen

RESULT=$(dialog --title "Penaz's (Mis)Guided Build System"\
    --ok-label "Build" --cancel-label "Abort"\
    --extra-button --extra-label "Build All"\
    --checklist --stdout "What do you want to build today?" 20 80 10\
    pseudocode "Pseudocode Edition (PDF)" on\
    epub_pseudocode "Pseudocode Edition (Epub)" off\
    python "Python Edition (PDF)" off\
    epub_python "Python Edition (Epub)" off\
    cpp "C++ Edition (PDF)" off\
    epub_cpp "C++ Edition (Epub)" off\
    js "JavaScript Edition (PDF)" off\
    epub_js "JavaScript Edition (Epub)" off\
    lua "Lua Edition (PDF)" off\
    epub_lua "Lua Edition (Epub)" off\
)

RETURN_VALUE=$?

clear

case "$RETURN_VALUE" in
    0)  # Build selected
        echo "Building...";
        make ${RESULT};;
    3)  # Build All Selected
        echo "Building all files...";
        MISSING_LISTINGS=$(./scripts/listing_status.py -q)
        if [ ${MISSING_LISTINGS} -ne 0 ];
        then
            echo "There are ${MISSING_LISTINGS} missing listings, cannot build all editions";
        else
            make all;
        fi;;
    1)  # Aborted
        echo "Build Aborted";
        exit 1;;
esac
