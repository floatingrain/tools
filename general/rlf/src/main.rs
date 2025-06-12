use anyhow::{Result};
use rayon::prelude::*;
use std::fs;
use std::io::{Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// 目标路径（文件或目录）
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let target_path = &args.path;

    let files = collect_files(target_path)?;
    println!("Found {} files to process", files.len());

    let processed_count = AtomicUsize::new(0);
    files.par_iter().for_each(|path| {
        if let Err(e) = process_file(path) {
            eprintln!("Error processing {}: {}", path.display(), e);
        } else {
            let count = processed_count.fetch_add(1, Ordering::Relaxed) + 1;
            println!("Processed {}/{}: {}", count, files.len(), path.display());
        }
    });

    println!("Conversion completed");
    Ok(())
}

fn collect_files(path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if path.is_file() {
        files.push(path.to_path_buf());
    } else if path.is_dir() {
        for entry in walkdir::WalkDir::new(path) {
            match entry {
                Ok(entry) => {
                    if entry.file_type().is_file() {
                        files.push(entry.into_path());
                    }
                }
                Err(e) => eprintln!("Error accessing file: {}", e),
            }
        }
    }

    Ok(files)
}

fn process_file(path: &Path) -> Result<()> {
    if !is_text_file(path)? {
        println!("Not UTF-8 text file: {}, skipping", path.display());
        return Ok(());
    }

    // 读取原始字节
    let mut file = fs::File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // 检查原始换行符
    let contains_crlf = contents.windows(2).any(|w| w == b"\r\n");
    let contains_cr = contents.contains(&b'\r');
    
    if !contains_crlf && !contains_cr {
        return Ok(()); // 不需要转换
    }

    // 执行CRLF/CR到LF的转换
    let mut new_contents = Vec::with_capacity(contents.len());
    let mut i = 0;
    
    while i < contents.len() {
        if contents[i] == b'\r' {
            // 处理CRLF (\r\n)
            if i + 1 < contents.len() && contents[i + 1] == b'\n' {
                new_contents.push(b'\n');
                i += 2; // 跳过CR和LF
                continue;
            } 
            // 处理单独的CR (\r)
            else {
                new_contents.push(b'\n');
            }
        } else {
            new_contents.push(contents[i]);
        }
        i += 1;
    }

    // 写回文件
    fs::write(path, &new_contents)?;
    Ok(())
}

fn is_text_file(path: &Path) -> Result<bool> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0; 1024];
    
    let bytes_read = file.read(&mut buffer)?;
    let content = &buffer[..bytes_read];
    
    // 空字节是二进制文件的明确标志
    if content.contains(&0) {
        return Ok(false);
    }
    
    // 允许的文本控制字符
    const ALLOWED_CONTROLS: &[u8] = &[9, 10, 12, 13]; // TAB, LF, FF, CR
    let mut control_count = 0;
    let mut byte_count = 0;
    
    match std::str::from_utf8(content) {
        Ok(_) => {
            for &byte in content {
                byte_count += 1;
                if byte <= 31 && !ALLOWED_CONTROLS.contains(&byte) {
                    control_count += 1;
                }
            }
        }
        Err(_) => {
            return Ok(false)
        }
    }
    
    // 计算控制字符比例
    let ratio = if byte_count > 0 {
        control_count as f32 / byte_count as f32
    } else {
        0.0
    };
    
    Ok(ratio < 0.1) // 控制字符占比小于10%
}