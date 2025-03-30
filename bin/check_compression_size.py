#!/usr/bin/env python

import sys
import os

# Small script to check the compressed files in the dist directory:
# check if all files have compressed versions
# check if compressed version is smaller than original one
# check if brotli version is smaller than gzip version
# (includes an ignore-list for some files and some file extensions)
# This script is meant to get a feeling about compression outliers

def  check_recursive(path):
    for (dirpath, _, filenames) in os.walk(path, topdown = False):
        filenames = set(filenames)
        for filename in filenames:
            (name, ext) = os.path.splitext(filename)
            if ext == ".gz" and os.path.splitext(name)[1] == ".tar":
                pass
            elif ext == ".gz" or ext == ".br":
                if name not in filenames:
                    print(f"'{os.path.join(dirpath, filename)}' has no uncompressed version")
            elif ext in [".jpg", ".jpeg", ".png", ".webp", ".pdf", "", ".dat", ".bz2", ".xml"]:
                pass
            elif filename in ["robots.txt"]:
                pass
            else:
                path = os.path.join(dirpath, filename)
                size = os.path.getsize(path)

                gzname = filename + ".gz"
                gzpath = os.path.join(dirpath, gzname)
                gsize = None
                if gzname in filenames:
                    gzsize = os.path.getsize(gzpath)
                    if gzsize > size:
                        print(f"{gzpath} is larger than {path}")
                else:
                    print(f"'{path}' has not gzipped version")

                brname = filename + ".br"
                brpath = os.path.join(dirpath, brname)
                brsize = None
                if brname in filenames:
                    brsize = os.path.getsize(brpath)
                    if brsize > size:
                        print(f"{brpath} is larger than {path}")
                else:
                    print(f"'{path}' has not brotli version")

                if brsize is not None and gzsize is not None:
                    if brsize > gzsize:
                        print(f"'{brpath}' is larger than '{gzpath}'")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(f"usage: {sys.argv[0]} <directory>", file=sys.stderr)
        sys.exit(1)

    directory = sys.argv[1]

    check_recursive(directory)
