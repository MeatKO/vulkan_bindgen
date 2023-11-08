#!/bin/bash

# Loop through each subdirectory of the current directory
for dir in */ ; do
    # Check if the subdirectory contains the shell script
    if [[ -f "${dir}shader.vert" && -f "${dir}shader.frag" ]]; then
        # Check if the .spv files do not exist or are older than the corresponding source files
        if [[ "${dir}shader.vert" -nt "${dir}vert.spv" ]] || [[ "${dir}shader.frag" -nt "${dir}frag.spv" ]]; then
            echo "Recompiling shaders in ${dir}..."
            glslc "${dir}shader.vert" -o "${dir}vert.spv"
            glslc "${dir}shader.frag" -o "${dir}frag.spv"
        else
            echo "Shaders in ${dir} are up to date."
        fi
    else
        echo "Shader source files not found in ${dir}, skipping..."
    fi
done