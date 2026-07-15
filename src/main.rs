use hound;
use clap::Parser;

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

    Ok(AnalysisResult {
        sample_rate: spec.sample_rate,
        channels: spec.channels,
        bits_per_sample: spec.bits_per_sample,
        sample_count: samples.len(),
        peek,
        rms,
    })
}

fn print_text(result: &AnalysisResult) {
    println!("サンプルレート: {}", result.sample_rate);
    println!("チャンネル数: {}", result.channels);
    println!("サンプル深度: {}", result.bits_per_sample);
    println!("サンプル数: {}", result.sample_count);
    println!("ピーク値: {}", result.peek);
    println!("RMS: {}", result.rms);
}

fn print_json(result: &AnalysisResult) {
    println!("{{");
    println!("  \"sample_rate\": {},", result.sample_rate);
    println!("  \"channels\": {},", result.channels);
    println!("  \"bits_per_sample\": {},", result.bits_per_sample);
    println!("  \"samples_count\": {},", result.sample_count);
    println!("  \"peek\": {},", result.peek);
    println!("  \"rms\": {}", result.rms);
    println!("}}");
}

fn main() {
    let args = Args::parse();

    match analyze(&args.path) {
        Ok(result) => {
            match args.format.as_str() {
                "json" => print_json(&result),
                _ => print_text(&result),
            }
        },
        Err(e) => println!("エラー: {}", e),
    }
}
