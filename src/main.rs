use hound;
use clap::Parser;
use walkdir::WalkDir;
use rayon::prelude::*;

#[derive(Parser)]
struct Args {
    path: String,
    #[arg(long, default_value = "text")]
    format: String,
}

struct AnalysisResult {
    sample_rate: u32,
    channels: u16,
    bits_per_sample: u16,
    sample_count: usize,
    peek: i32,
    rms: f64,
    warnings: Vec<String>
}

fn analyze(path: &str) -> Result<AnalysisResult, String> {
    let mut reader = hound::WavReader::open(path)
        .map_err(|e| format!("ファイルを開けませんでした: {}", e))?;
    let spec = reader.spec();

    let samples: Vec<i32> = reader.samples::<i32>()
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| format!("サンプルの読み込みに失敗しました: {}", e))?;

    let peek = samples.iter().map(|s| s.abs()).max().unwrap_or(0);

    let sum_of_squares: f64 = samples.iter().map(|s| (*s as f64).powi(2)).sum();
    let rms = (sum_of_squares / samples.len() as f64).sqrt();

    let mut warnings: Vec<String> = Vec::new();

    if rms < 1.0 {
        warnings.push("無音の可能性があります".to_string())
    }

    // 32767 * 95% = 31128
    if peek > 31128 {
        warnings.push("クリップの可能性があります".to_string())
    }

    Ok(AnalysisResult {
        sample_rate: spec.sample_rate,
        channels: spec.channels,
        bits_per_sample: spec.bits_per_sample,
        sample_count: samples.len(),
        peek,
        rms,
        warnings,
    })
}

fn print_text(result: &AnalysisResult) {
    println!("サンプルレート: {}", result.sample_rate);
    println!("チャンネル数: {}", result.channels);
    println!("サンプル深度: {}", result.bits_per_sample);
    println!("サンプル数: {}", result.sample_count);
    println!("ピーク値: {}", result.peek);
    println!("RMS: {}", result.rms);
    for w in &result.warnings {
        println!("⚠ {}", w);
    }
}

fn print_json(result: &AnalysisResult) {
    println!("{{");
    println!("  \"sample_rate\": {},", result.sample_rate);
    println!("  \"channels\": {},", result.channels);
    println!("  \"bits_per_sample\": {},", result.bits_per_sample);
    println!("  \"samples_count\": {},", result.sample_count);
    println!("  \"peek\": {},", result.peek);
    println!("  \"rms\": {},", result.rms);
    let warnings_json = result.warnings.iter()
        .map(|w| format!("\"{}\"", w))
        .collect::<Vec<String>>()
        .join(", ");
    println!("  \"warnings\": [{}]", warnings_json);
    println!("}}");
}

fn find_wav_files(dir: &str) -> Vec<String> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().to_string_lossy().to_string().ends_with(".wav"))
        .map(|e| e.path().to_string_lossy().to_string())
        .collect()
}

fn main() {
    let args = Args::parse();

    let files = find_wav_files(&args.path);
    println!("見つかったWAVファイル数: {}", files.len());

    files.par_iter().for_each(|path| {
        match analyze(&path) {
            Ok(result) => {
                println!("--- {} ---", path);
                match args.format.as_str() {
                    "json" => print_json(&result), 
                    _ => print_text(&result),
                }
            },
            Err(e) => println!("エラー: {}", e),
        }
    });
}
