use std::env;
use std::fs;
use std::process::Command;

fn main() {
    println!("Running build.rs...");

    // Get project and output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("OUT_DIR: {}", out_dir);
    println!("Project directory: {}", project_dir);

    // Compile boot_stage2 with BOTH boot2 AND startup feature flags
    println!("Compiling boot stage 2...");
    let status = Command::new("rustc")
        .args(&[
            "--crate-type=lib",
            "--emit=obj",
            "--target=thumbv6m-none-eabi",
            "-C", "opt-level=s",
            "-C", "link-arg=-nostartfiles",
            "-C", "panic=abort",
            "--cfg", "feature=\"boot2\"",  // Add boot2 feature flag
            "--cfg", "feature=\"startup\"",  // ADDED: Also enable startup feature
            "-o", &format!("{}/boot2.o", out_dir),
            &format!("{}/src/lib.rs", project_dir),
        ])
        .status()
        .expect("Failed to compile boot stage 2");

    if !status.success() {
        panic!("Failed to compile boot stage 2");
    }
    println!("Boot stage 2 compiled successfully.");

    // Generate boot2 binary using arm-none-eabi-objcopy
    println!("Generating boot2 binary...");
    let boot2_bin = format!("{}/boot2.bin", out_dir);
    let output = Command::new("arm-none-eabi-objcopy")
        .args(&[
            "-O", "binary",
            &format!("{}/boot2.o", out_dir),
            &boot2_bin,
        ])
        .output()
        .expect("Failed to execute arm-none-eabi-objcopy");

    if !output.status.success() {
        eprintln!("arm-none-eabi-objcopy failed: {}", String::from_utf8_lossy(&output.stderr));
        panic!("arm-none-eabi-objcopy failed");
    }
    println!("Boot2 binary generated successfully.");

    // Ensure boot2.bin is properly sized (252 bytes)
    let bin_data = fs::read(&boot2_bin).expect("Failed to read boot2.bin");
    let mut data = bin_data.clone();
    data.resize(252, 0); // Pad to 252 bytes

    // Calculate CRC32
    use crc::{Crc, CRC_32_MPEG_2};
    let crc = Crc::<u32>::new(&CRC_32_MPEG_2);
    let crc_value = crc.checksum(&data);

    // Convert to little-endian bytes
    let crc_bytes = crc_value.to_le_bytes();

    // Append CRC to final binary
    let mut final_bin = data;
    final_bin.extend_from_slice(&crc_bytes);

    // Write final boot2.bin with CRC
    println!("Writing final boot2.bin with CRC...");
    fs::write(&boot2_bin, &final_bin).expect("Failed to write boot2.bin with CRC");

    // Ensure boot2.bin is exactly 256 bytes
    if final_bin.len() != 256 {
        panic!(
            "Error: boot2.bin is {} bytes, but should be exactly 256 bytes",
            final_bin.len()
        );
    }

    println!(
        "Boot2 CRC: {:02x}{:02x}{:02x}{:02x}",
        crc_bytes[0], crc_bytes[1], crc_bytes[2], crc_bytes[3]
    );

    println!("Compiling startup...");
    let status = Command::new("rustc")
        .args(&[
            "--crate-type=lib",
            "--emit=obj",
            "--target=thumbv6m-none-eabi",
            "-C", "opt-level=s",
            "-C", "link-arg=-nostartfiles",
            "-C", "panic=abort",
            "-C", "debuginfo=2",
            "--cfg", "feature=\"startup\"",  
            "-o", &format!("{}/startup.o", out_dir),
            &format!("{}/src/lib.rs", project_dir),
        ])
        .status()
        .expect("Failed to compile startup");

    if !status.success() {
        panic!("Failed to compile startup");
    }
    println!("Startup compiled successfully.");

    println!("Compiling transmit...");
    let status = Command::new("rustc")
        .args(&[
            "--crate-type=lib",
            "--emit=obj",
            "--target=thumbv6m-none-eabi",
            "-C", "opt-level=s",
            "-C", "link-arg=-nostartfiles",
            "-C", "panic=abort",
            "-C", "debuginfo=2",
            "--cfg", "feature=\"transmit\"",  // Add transmit feature flag
            "-o", &format!("{}/transmit.o", out_dir),
            &format!("{}/src/lib.rs", project_dir),
        ])
        .status()
        .expect("Failed to compile transmit");

    if !status.success() {
        panic!("Failed to compile transmit");
    }
    println!("Transmit compiled successfully.");

    // Create the correct sections in the final ELF by using a proper memory.x linker script
    // Make sure the linker script is properly passed
    println!("cargo:rustc-link-search={}", project_dir);

    // Link all object files into a final ELF file
    println!("Linking ELF...");
    let status = Command::new("arm-none-eabi-gcc")
        .args(&[
            "-mcpu=cortex-m0plus", 
            "-nostdlib",
            "-g",
            &format!("-T{}/memory.x", project_dir),  // Full path to memory.x
            "-o", &format!("{}/transmitter.elf", out_dir),
            &format!("{}/boot2.o", out_dir),
            &format!("{}/startup.o", out_dir),
            &format!("{}/transmit.o", out_dir),
            "-Wl,-Map=output.map",
            "-Wl,--allow-multiple-definition"  
        ])
        .status()
        .expect("Failed to link ELF file");

    if !status.success() {
        panic!("Failed to link ELF file");
    }
    println!("ELF linked successfully.");

    // Convert ELF to binary first
    println!("Converting ELF to binary...");
    let transmitter_bin = format!("{}/transmitter.bin", out_dir);
    let status = Command::new("arm-none-eabi-objcopy")
        .args(&[
            "-O", "binary",
            &format!("{}/transmitter.elf", out_dir),
            &transmitter_bin,
        ])
        .status()
        .expect("Failed to convert ELF to binary");

    if !status.success() {
        panic!("Failed to convert ELF to binary");
    }

    // Convert binary to UF2 using python script with -c flag
    println!("Generating UF2 file...");
    let __ = Command::new("python3")
        .args(&[
            "tools/uf2/utils/uf2conv.py", 
            "-b", "0x10000000",  // Base address for RP2040
            "-f", "0xe48bff56",  // RP2040 family ID
            "-c",  // Specify that input is a BIN file
            &transmitter_bin, 
            "-o", &format!("{}/transmitter.uf2", out_dir)
        ])
        .output()
        .expect("Failed to convert to UF2");

    // Copy the final UF2 file to the project directory
    let final_uf2 = format!("{}/transmitter.uf2", out_dir);
    
    // Use fs instead of the cp command for better cross-platform compatibility
    fs::copy(&final_uf2, format!("{}/transmitter.uf2", project_dir))
        .expect("Failed to copy UF2 file");

    println!("UF2 file copied to project directory.");

    // Tell Cargo to rebuild if the source files change
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}