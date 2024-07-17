use encoding_rs::*;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::runtime::Runtime;
use tokio::task;

#[derive(StructOpt)]
struct Cli {
    /// 输入文件或目录
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    /// 输出文件或目录
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,

    /// 源编码
    #[structopt(short, long)]
    from: String,

    /// 目标编码
    #[structopt(short, long)]
    to: String,
}

async fn convert_file(
    input_path: PathBuf,
    output_path: PathBuf,
    source_encoding: String,
    target_encoding: String,
) -> io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    let source_coder = Encoding::for_label(source_encoding.as_bytes())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "无效的源编码"))?;
    let target_coder = Encoding::for_label(target_encoding.as_bytes())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "无效的目标编码"))?;

    let (decoded, _, _) = source_coder.decode(&buffer);
    let (encoded, _, _) = target_coder.encode(&decoded);

    let mut output_file = File::create(output_path)?;
    output_file.write_all(&encoded)?;

    Ok(())
}

async fn process_files(
    input_path: PathBuf,
    output_path: PathBuf,
    source_encoding: String,
    target_encoding: String,
) -> io::Result<()> {
    let mut tasks = Vec::new();

    if input_path.is_dir() {
        fs::create_dir_all(&output_path)?;

        for entry in fs::read_dir(input_path)? {
            let entry = entry?;
            let input_file_path = entry.path();
            let output_file_path = output_path.join(entry.file_name());

            let source_encoding_clone = source_encoding.clone();
            let target_encoding_clone = target_encoding.clone();

            tasks.push(task::spawn(async move {
                if let Err(e) = convert_file(
                    input_file_path,
                    output_file_path,
                    source_encoding_clone,
                    target_encoding_clone,
                )
                .await
                {
                    eprintln!("Failed to convert file: {}", e);
                }
            }));
        }
    } else {
        let output_file_path = output_path;

        tasks.push(task::spawn(async move {
            if let Err(e) = convert_file(
                input_path,
                output_file_path,
                source_encoding,
                target_encoding,
            )
            .await
            {
                eprintln!("Failed to convert file: {}", e);
            }
        }));
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    let runtime = Runtime::new()?;
    runtime.block_on(async { process_files(args.input, args.output, args.from, args.to).await })
}
