alias b := build
alias d := doc
alias f := format

# List recipes by default
_default:
    @just --list --unsorted

# Build source and docs
build TARGET:
    #!/bin/sh
    case "{{TARGET}}" in
        "release" | "r")
            cargo build --release
            cargo doc --release
            ;;
        "debug" | "d")
            cargo build
            cargo doc
            ;;
        *)
            echo "Invalid target."
            ;;
    esac

# Build and open docs
doc TARGET:
    #!/bin/sh
    case "{{TARGET}}" in
        "mdbook" | "book" | "b")
            mdbook serve --open docs
            ;;
        "cargo" | "c")
            cargo doc --open
            ;;
        *)
            echo "Invalid target."
    esac

format:
    cargo fmt
    mdformat docs --wrap 80
    mdformat README.md --wrap 80
