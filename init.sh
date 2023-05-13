#!/bin/env bash
# Creates a virtualenv in $HOME/.virtualenvs
# Init and pull submodules
# Download execution-specs dependencies and fixtures for tests
# Makes available test data for rust-execution-specs

VIRTUALENVS_FOLDER=$HOME/.virtualenvs


VIRTUALENVS_EXECUTION_SPECS_FOLDER=$VIRTUALENVS_FOLDER/exection-specs

EXECUTION_SPECS_FOLDER=./execution-specs
RUST_EXECUTION_SPECS_FOLDER=./rust-execution-specs

if [[ ! -d "$VIRTUALENVS_FOLDER" ]]; then
    echo "Please, install virtualenv wrapper (https://virtualenvwrapper.readthedocs.io/en/latest/install.html#basic-installation)."
    echo "   $ python -m pip install --user virtualenv"
    echo "   $ python -m pip install --user virtualenvwrapper"
    echo "   # Add three lines to your shell startup file (.bashrc, .profile, etc.)"
    echo "   export WORKON_HOME=$HOME/.virtualenvs"
    echo "   export PROJECT_HOME=$HOME/Devel # update with your project folder"
    echo "   source /usr/local/bin/virtualenvwrapper.sh"
    exit 1
fi

if [[ ! -d "$VIRTUALENVS_EXECUTION_SPECS_FOLDER" ]]; then
    virtualenv $VIRTUALENVS_EXECUTION_SPECS_FOLDER
fi

if [[ ! "$EXECUTION_SPECS_FOLDER/tests/fixtures" ]]; then
    source $VIRTUALENVS_EXECUTION_SPECS_FOLDER/bin/activate
    
    pip install pytest gitpython filelock
    git submodule init
    git submodule update --recursive --remote
    cd execution-specs
    pip install .
    pytest --setup-only --collect-only
    cd ..

    deactivate
fi

if [[ ! -L "$RUST_EXECUTION_SPECS_FOLDER/tests/fixtures" ]]; then
    echo "[INFO] Creating fixtures link"
    ln -sf $(pwd)/$EXECUTION_SPECS_FOLDER/tests/fixtures/ $(pwd)/$RUST_EXECUTION_SPECS_FOLDER/tests/fixtures 
fi