#!/bin/bash

template="templates/default.stub.rs"
output_dir="../src"

# input number
read -p "Input number: " number

# padding zero to left
formatted_number=$(printf "%03d" $number)

# input title
read -p "Input title: " title

# convert snake_case
filename=$(echo "$title" | sed -E 's/[^a-zA-Z0-9]+/_/g' | sed -E 's/(^_|_$)//g' | tr 'A-Z' 'a-z')

if [ -n "$template" ] && [ -n "$filename" ]; then
  # make directory
  mkdir -p "${output_dir}/${formatted_number}"
  # file copy
  cp "$template" "${output_dir}/${formatted_number}/${filename}.rs"
  # check file exists
  if [ $? -eq 0 ]; then
    echo "File created."
  else
    echo "Failed to copy the file..."
  fi
else 
  echo "Invalid input. Please provide a template and a filename."
fi