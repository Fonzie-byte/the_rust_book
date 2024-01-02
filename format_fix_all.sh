#!/bin/bash

# For each directory...
for directory in $(ls -d */); do
	# If file "Cargo.toml" is missing there...
	if [ ! -f "${directory}Cargo.toml" ]; then
		# Tell them this is not cargo.
		echo "No cargo found, skipping ${directory}\n"
		# Continue to the next directory.
		continue
	fi

	# Otherwise, tell them we're fixing it.
	echo "Formatting, fixing and releasing ${directory}"

	# Go there.
	cd $directory
	# Make sure everything will compile.
	cargo check && \
	# If so, format the code.
	cargo fmt && \
	# Fix low-hanging fruit.
	cargo fix --allow-no-vcs && \
	# Find possible improvements.
	cargo clippy
	# Go back up, to fix the next one.
	cd ../

	# Echo blank line, to improve legibility.
	echo
done
