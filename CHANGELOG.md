# Changelog

## 0.1.11

1. Updated `tempfile` dev dependency from `3.20.0` to `3.22.0`.
1. Added `slab` dependency at `0.4.11` (since `0.4.10` is yanked).

## 0.1.10

1. Improved documentation.

## 0.1.9

1. Added `copy_folder`, `print_folder_tree`, and `list_folder_contents` functions.

## 0.1.8

1. Reduced the length of the printout to `stderr` when `replace_str_in_files` hits an error.

## 0.1.7

1. Updated `replace_str_in_files` to print a warning to `stderr` instead of `panicking` when it fails to write to a file.

## 0.1.6

1. Added `copy_file` and `delete_file` functions.

## 0.1.5

1. Added `get_file_extension`, `delete_folder`, and `to_path_buf` functions.
1. Updated the `cd` function to return a `CdGuard` which automatically restores the current working directory when it moves out of a scope.

## 0.1.4

1. Added `get_file_name` and `get_file_stem` functions.

## 0.1.3

1. Fixed all references to the GitHub repository after it was renamed from `file-io` to `file_io`.

## 0.1.2

1. Better error messages.

## 0.1.1

1. Fix README.

## 0.1.0

1. Initial release.