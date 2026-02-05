set SCRIPT_DIR (dirname (status --current-filename))
cd $SCRIPT_DIR

fish run_rust_tests.fish
fish run_python_tests.fish
