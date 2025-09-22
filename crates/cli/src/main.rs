//! Package Fast CLI - Command line interface for Package Fast

use anyhow::Result;
use clap::Parser;
use package_fast_core::{install_all_dependencies, install_packages, InstallOptions};
use tracing_subscriber;

/// Package Fast - A very fast Node.js package manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Subcommands
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Install dependencies
    Install {
        /// Install dev dependencies only
        #[arg(short = 'D', long)]
        dev: bool,

        /// Install production dependencies only
        #[arg(short = 'P', long)]
        prod: bool,

        /// Force reinstall packages
        #[arg(short, long)]
        force: bool,

        /// Packages to install
        packages: Vec<String>,
    },
    
    /// Add packages to dependencies
    Add {
        /// Add to devDependencies
        #[arg(short = 'D', long)]
        dev: bool,

        /// Packages to add
        packages: Vec<String>,
    },
    
    /// Remove packages from dependencies
    Remove {
        /// Packages to remove
        packages: Vec<String>,
    },
    
    /// Update packages
    Update {
        /// Packages to update
        packages: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let args = Args::parse();
    
    match &args.command {
        Some(Commands::Install { dev, prod, force, packages }) => {
            let options = InstallOptions {
                dev_only: *dev,
                prod_only: *prod,
                force: *force,
            };
            
            if packages.is_empty() {
                println!("Installing all dependencies from package.json");
                let result = install_all_dependencies(&options).await?;
                println!("Installed {} packages", result.installed_packages.len());
            } else {
                println!("Installing packages: {:?}", packages);
                let result = install_packages(packages, &options).await?;
                println!("Installed {} packages", result.installed_packages.len());
            }
        }
        Some(Commands::Add { dev, packages }) => {
            println!("Adding packages: {:?}", packages);
            if *dev {
                println!("Adding to devDependencies");
            } else {
                println!("Adding to dependencies");
            }
            // TODO: Implement add logic
        }
        Some(Commands::Remove { packages }) => {
            println!("Removing packages: {:?}", packages);
            // TODO: Implement removal logic
        }
        Some(Commands::Update { packages }) => {
            println!("Updating packages: {:?}", packages);
            // TODO: Implement update logic
        }
        None => {
            println!("No command provided. Use --help for usage information.");
        }
    }
    
    Ok(())
}