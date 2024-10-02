use clap::{Arg, ArgAction, Command};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write}; // Added Read and Write traits
use std::path::Path;
use walkdir::WalkDir;
use dialoguer::{theme::ColorfulTheme, MultiSelect}; // Changed Select to MultiSelect

fn main() {
    // Define command-line arguments using clap
    let matches = Command::new("dupehunter")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("Finds and manages duplicate files in directories")
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue)
                .help("Recursively search through subdirectories"),
        )
        .arg(
            Arg::new("ftype")
                .long("ftype")
                .value_name("FILETYPE")
                .num_args(1)
                .help("Specify file types to include (e.g., mp3, mp4). Comma-separated for multiple types."),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .value_name("DIRECTORY")
                .num_args(1)
                .required(true)
                .help("Specify the target directory to scan (e.g., ./)"),
        )
        .arg(
            Arg::new("interactive")
                .long("interactive")
                .action(ArgAction::SetTrue)
                .help("Enable interactive mode for deleting duplicates"),
        )
        .arg(
            Arg::new("auto_delete")
                .long("auto-delete")
                .action(ArgAction::SetTrue)
                .help("Automatically delete duplicates without confirmation"),
        )
        .arg(
            Arg::new("dry_run")
                .long("dry-run")
                .action(ArgAction::SetTrue)
                .help("Preview duplicates without deleting them"),
        )
        .arg(
            Arg::new("report")
                .long("report")
                .value_name("REPORT_FILE")
                .num_args(1)
                .help("Generate a report of duplicates to the specified file"),
        )
        .arg(
            Arg::new("ignore_hidden")
                .long("ignore-hidden")
                .action(ArgAction::SetTrue)
                .help("Ignore hidden files and directories"),
        )
        .get_matches();

    // Extract arguments
    let recursive = matches.get_flag("recursive");
    let ftype = matches.get_one::<String>("ftype").map(|s| {
        s.split(',')
            .map(|ext| ext.trim().to_lowercase())
            .collect::<Vec<String>>()
    });
    let dir = matches.get_one::<String>("dir").unwrap();
    let interactive = matches.get_flag("interactive");
    let auto_delete = matches.get_flag("auto_delete");
    let dry_run = matches.get_flag("dry_run");
    let report = matches.get_one::<String>("report");
    let ignore_hidden = matches.get_flag("ignore_hidden");

    // Validate the directory path
    let dir_path = Path::new(dir);
    if !dir_path.is_dir() {
        eprintln!("Error: The specified path is not a directory.");
        std::process::exit(1);
    }

    // Traverse directories
    let walker = WalkDir::new(dir_path)
        .follow_links(false)
        .min_depth(1)
        .max_depth(if recursive { usize::MAX } else { 1 })
        .into_iter()
        .filter_entry(|e| {
            if ignore_hidden {
                !is_hidden(e)
            } else {
                true
            }
        });

    // Map to store files grouped by size
    let mut files_by_size: HashMap<u64, Vec<std::path::PathBuf>> = HashMap::new();

    println!("Scanning files...");

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    if let Some(ref ftypes) = ftype {
                        if let Some(ext) = path.extension() {
                            if !ftypes.contains(&ext.to_string_lossy().to_lowercase()) {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    }

                    match fs::metadata(path) {
                        Ok(metadata) => {
                            let size = metadata.len();
                            files_by_size.entry(size).or_default().push(path.to_path_buf());
                        }
                        Err(e) => eprintln!("Could not access {}: {}", path.display(), e),
                    }
                }
            }
            Err(e) => eprintln!("Error accessing entry: {}", e),
        }
    }

    println!("Grouping potential duplicates...");

    // Map to store files grouped by hash
    let mut files_by_hash: HashMap<String, Vec<std::path::PathBuf>> = HashMap::new();

    for (size, files) in files_by_size.iter().filter(|(_, v)| v.len() > 1) {
        for file in files {
            match hash_file(file) {
                Ok(hash) => {
                    files_by_hash.entry(hash).or_default().push(file.clone());
                }
                Err(e) => eprintln!("Failed to hash {}: {}", file.display(), e),
            }
        }
    }

    // Collect duplicates
    let duplicates: Vec<Vec<std::path::PathBuf>> = files_by_hash
        .values()
        .filter(|group| group.len() > 1)
        .cloned()
        .collect();

    if duplicates.is_empty() {
        println!("No duplicate files found.");
        return;
    }

    println!("Found {} groups of duplicates.", duplicates.len());

    // Generate report if requested
    if let Some(report_path) = report {
        if let Err(e) = generate_report(&duplicates, report_path) {
            eprintln!("Failed to generate report: {}", e);
        } else {
            println!("Report generated at {}", report_path);
        }
    }

    // Handle duplicates
    if dry_run {
        println!("Dry run mode enabled. The following duplicates would be deleted:");
        for group in &duplicates {
            for file in &group[1..] {
                println!("{}", file.display());
            }
        }
    } else if auto_delete || interactive {
        for group in &duplicates {
            if group.len() < 2 {
                continue;
            }

            let originals = &group[0];
            let duplicates_files = &group[1..];

            if interactive {
                println!("\nDuplicate Group:");
                println!("Original: {}", originals.display());
                println!("Duplicates:");
                for (i, file) in duplicates_files.iter().enumerate() {
                    println!("  [{}] {}", i + 1, file.display());
                }

                let selection = MultiSelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select files to delete (Use space to select, enter to confirm)")
                    .items(&duplicates_files.iter().map(|f| f.to_string_lossy().to_string()).collect::<Vec<String>>())
                    .interact()
                    .unwrap();

                for index in selection {
                    if let Some(file) = duplicates_files.get(index) {
                        match fs::remove_file(file) {
                            Ok(_) => println!("Deleted: {}", file.display()),
                            Err(e) => eprintln!("Failed to delete {}: {}", file.display(), e),
                        }
                    }
                }
            } else if auto_delete {
                for file in duplicates_files {
                    match fs::remove_file(file) {
                        Ok(_) => println!("Deleted: {}", file.display()),
                        Err(e) => eprintln!("Failed to delete {}: {}", file.display(), e),
                    }
                }
            }
        }
    } else {
        println!("No action taken. Use --interactive or --auto-delete to remove duplicates.");
    }

    println!("Duplicate hunting complete.");
}

// Function to check if a file or directory is hidden
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

// Function to hash a file's contents using SHA256
fn hash_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    let hash = hasher.finalize();
    Ok(hex::encode(hash))
}

// Function to generate a report of duplicates
fn generate_report(
    duplicates: &Vec<Vec<std::path::PathBuf>>,
    report_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut report = fs::File::create(report_path)?;

    for (i, group) in duplicates.iter().enumerate() {
        writeln!(report, "Duplicate Group {}:", i + 1)?;
        for file in group {
            writeln!(report, "  {}", file.display())?;
        }
        writeln!(report)?;
    }

    Ok(())
}
