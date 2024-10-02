# dupehunter
more adjusting to rustlang


DupeHunter

DupeHunter is a powerful and efficient Rust-based command-line tool designed to help users find and manage duplicate files across their system. Whether you're looking to free up disk space, organize your directories, or maintain a clean file structure, DupeHunter provides the necessary features to streamline the process.

Table of Contents
Features
Installation
Usage
Basic Duplicate Search
Recursive Scan for Specific File Types
Interactive Deletion of Duplicates
Automatic Deletion of Duplicates
Dry Run Mode
Generating a Report
Ignoring Hidden Files and Directories
Safety Precautions
Examples
Contributing
License
Acknowledgements
Features
Recursive Duplicate Search: Scan directories and their subdirectories to identify duplicate files.
File Size & Hash Comparison: Efficiently detect duplicates by comparing file sizes and confirming with SHA256 hashes.
File Type Filtering: Target specific file types (e.g., .mp3, .mp4) to narrow down the search.
Interactive Deletion Mode: Review and selectively delete duplicates through an intuitive interactive interface.
Batch Deletion: Automatically delete all duplicates without manual intervention.
Report Generation: Generate detailed reports listing all found duplicates for review or record-keeping.
Ignore Hidden/System Files: Optionally exclude hidden or system files from the scan to streamline results.
Dry Run Mode: Preview potential deletions without making any changes to your files.
Installation
Prerequisites
Rust: Ensure you have Rust installed. If not, install it from rustup.rs.
Building from Source
Clone the Repository

git clone https://github.com/dassC/dupehunter.git
cd dupehunter
Build the Project

cargo build --release
The compiled binary will be located at target/release/dupehunter.

Install Globally

To make dupehunter accessible from anywhere in your terminal:

cargo install --path .
Note: Ensure that ~/.cargo/bin is included in your system's PATH environment variable.

Add the following line to your shell configuration file (e.g., .bashrc, .zshrc):

export PATH="$HOME/.cargo/bin:$PATH"
Then, reload your shell configuration:

source ~/.bashrc
# or
source ~/.zshrc
Using Precompiled Binaries
Coming Soon: If you prefer using precompiled binaries, downloads will be available in the Releases section.

Usage
DupeHunter offers a variety of command-line options to tailor the duplicate search and management process to your needs.

1. Basic Duplicate Search
Scan the current directory non-recursively and list duplicate files.

dupehunter --dir ./
2. Recursive Scan for Specific File Types
Recursively scan for .mp3 and .mp4 files and identify duplicates.

dupehunter -r --ftype mp3,mp4 --dir ./Music
3. Interactive Deletion of Duplicates
Find duplicates and interactively choose which ones to delete.

dupehunter -r --dir ./Documents --interactive
4. Automatic Deletion of Duplicates
Automatically delete all duplicate files without prompting.

dupehunter -r --dir ./Downloads --auto-delete
5. Dry Run Mode
Preview which duplicates would be deleted without performing any deletions.

dupehunter -r --dir ./Photos --dry-run
6. Generating a Report
Create a report of all found duplicates and save it to duplicates_report.txt.

dupehunter -r --dir ./Projects --report duplicates_report.txt
7. Ignoring Hidden Files and Directories
Exclude hidden files and directories from the duplicate search.

dupehunter -r --dir ./Workspace --ignore-hidden
Safety Precautions
⚠️ Warning: DupeHunter performs file deletions, which are irreversible. To prevent accidental data loss:

Use Dry Run Mode First: Always run DupeHunter with the --dry-run flag to review which files will be affected.

dupehunter -r --dir ./ImportantFolder --dry-run
Backup Important Data: Before performing deletions, ensure that you have backups of critical files.

Review Deletions: In interactive mode, carefully select which duplicates to delete.

Examples
Example 1: Find and Preview Deletions
Find duplicate .jpg files in the ./Images directory and preview which files would be deleted.

dupehunter -r --ftype jpg --dir ./Images --dry-run
Example 2: Automatically Delete Duplicates in Videos Directory
Recursively scan the ./Videos directory for duplicates and delete them automatically.

dupehunter -r --dir ./Videos --auto-delete
Example 3: Generate a Duplicate Report
Scan the ./Projects directory for duplicates and generate a report.

dupehunter -r --dir ./Projects --report duplicates_report.txt

License
Distributed under the MIT License. See LICENSE for more information.
