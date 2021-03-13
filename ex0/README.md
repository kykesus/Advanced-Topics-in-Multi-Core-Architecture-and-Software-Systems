# Exercise #0 - Parallel Sort #

This is an implementation proposal for parallel sorting algorithm.
The chosen algorithm is k-way mergesort. The program expects a number of threads to use and an input file path containing list of 64-bits numbers.
The list is split into k parts, each being sorted by a different thread and finally all sorted parts are merged back to a complete sorted vector.

To build the code on rack-mad-01 server, we have to use udocker (cargo is not available on this server...). The build script pulls a rust docker image, creates a container and runs it with cargo build command. It then copies the artifacts to the directory from which the script was called and cleans up the environment.

### Usage
```bash
parsort [threads_num] [filename]
```