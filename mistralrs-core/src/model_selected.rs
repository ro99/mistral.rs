use clap::Subcommand;

use crate::{
    pipeline::{IsqOrganization, NormalLoaderType, VisionLoaderType},
    ModelDType,
};

fn parse_arch(x: &str) -> Result<NormalLoaderType, String> {
    x.parse()
}

fn parse_vision_arch(x: &str) -> Result<VisionLoaderType, String> {
    x.parse()
}

fn parse_model_dtype(x: &str) -> Result<ModelDType, String> {
    x.parse()
}

#[derive(Debug, Subcommand)]
pub enum ModelSelected {
    /// Select the model from a toml file
    Toml {
        /// .toml file containing the selector configuration.
        #[arg(short, long)]
        file: String,
    },

    /// Select a plain model, without quantization or adapters
    Plain {
        /// Model ID to load from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        model_id: String,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(short, long)]
        tokenizer_json: Option<String>,

        /// The architecture of the model.
        #[arg(short, long, value_parser = parse_arch)]
        arch: Option<NormalLoaderType>,

        /// Model data type. Defaults to `auto`.
        #[arg(short, long, default_value_t = ModelDType::Auto, value_parser = parse_model_dtype)]
        dtype: ModelDType,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,

        /// ISQ organization: `default` or `moqe` (Mixture of Quantized Experts: https://arxiv.org/abs/2310.02410).
        #[arg(short, long)]
        organization: Option<IsqOrganization>,
    },

    /// Select an X-LoRA architecture
    XLora {
        /// Force a base model ID to load from instead of using the ordering file. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        model_id: Option<String>,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(short, long)]
        tokenizer_json: Option<String>,

        /// Model ID to load X-LoRA from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        xlora_model_id: String,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// Index of completion tokens to generate scalings up until. If this is 1, then there will be one completion token generated before it is cached.
        /// This makes the maximum running sequences 1.
        #[arg(long)]
        tgt_non_granular_index: Option<usize>,

        /// The architecture of the model.
        #[arg(short, long, value_parser = parse_arch)]
        arch: Option<NormalLoaderType>,

        /// Model data type. Defaults to `auto`.
        #[arg(short, long, default_value_t = ModelDType::Auto, value_parser = parse_model_dtype)]
        dtype: ModelDType,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,
    },

    /// Select a LoRA architecture
    Lora {
        /// Force a base model ID to load from instead of using the ordering file. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        model_id: Option<String>,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(short, long)]
        tokenizer_json: Option<String>,

        /// Model ID to load LoRA from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        adapters_model_id: String,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// The architecture of the model.
        #[arg(long, value_parser = parse_arch)]
        arch: Option<NormalLoaderType>,

        /// Model data type. Defaults to `auto`.
        #[arg(short, long, default_value_t = ModelDType::Auto, value_parser = parse_model_dtype)]
        dtype: ModelDType,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,
    },

    /// Select a GGUF model.
    GGUF {
        /// `tok_model_id` is the local or remote model ID where you can find a `tokenizer_config.json` file.
        /// If the `chat_template` is specified, then it will be treated as a path and used over remote files,
        /// removing all remote accesses.
        #[arg(short, long)]
        tok_model_id: Option<String>,

        /// Quantized model ID to find the `quantized_filename`.
        /// This may be a HF hub repo or a local path.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename(s).
        /// May be a single filename, or use a delimiter of " " (a single space) for multiple files.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,
    },

    /// Select a GGUF model with X-LoRA.
    XLoraGGUF {
        /// `tok_model_id` is the local or remote model ID where you can find a `tokenizer_config.json` file.
        /// If the `chat_template` is specified, then it will be treated as a path and used over remote files,
        /// removing all remote accesses.
        #[arg(short, long)]
        tok_model_id: Option<String>,

        /// Quantized model ID to find the `quantized_filename`.
        /// This may be a HF hub repo or a local path.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename(s).
        /// May be a single filename, or use a delimiter of " " (a single space) for multiple files.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Model ID to load X-LoRA from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        xlora_model_id: String,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// Index of completion tokens to generate scalings up until. If this is 1, then there will be one completion token generated before it is cached.
        /// This makes the maximum running sequences 1.
        #[arg(long)]
        tgt_non_granular_index: Option<usize>,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,
    },

    /// Select a GGUF model with LoRA.
    LoraGGUF {
        /// `tok_model_id` is the local or remote model ID where you can find a `tokenizer_config.json` file.
        /// If the `chat_template` is specified, then it will be treated as a path and used over remote files,
        /// removing all remote accesses.
        #[arg(short, long)]
        tok_model_id: Option<String>,

        /// Quantized model ID to find the `quantized_filename`.
        /// This may be a HF hub repo or a local path.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename(s).
        /// May be a single filename, or use a delimiter of " " (a single space) for multiple files.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Model ID to load LoRA from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        adapters_model_id: String,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,
    },

    /// Select a GGML model.
    GGML {
        /// Model ID to load the tokenizer from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        tok_model_id: String,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(long)]
        tokenizer_json: Option<String>,

        /// Quantized model ID to find the `quantized_filename`.
        /// This may be a HF hub repo or a local path.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// GQA value
        #[arg(short, long, default_value_t = 1)]
        gqa: usize,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,
    },

    /// Select a GGML model with X-LoRA.
    XLoraGGML {
        /// Model ID to load the tokenizer from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        tok_model_id: Option<String>,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(long)]
        tokenizer_json: Option<String>,

        /// Quantized model ID to find the `quantized_filename`.
        /// This may be a HF hub repo or a local path.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Model ID to load X-LoRA from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        xlora_model_id: String,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// Index of completion tokens to generate scalings up until. If this is 1, then there will be one completion token generated before it is cached.
        /// This makes the maximum running sequences 1.
        #[arg(long)]
        tgt_non_granular_index: Option<usize>,

        /// GQA value
        #[arg(short, long, default_value_t = 1)]
        gqa: usize,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,
    },

    /// Select a GGML model with LoRA.
    LoraGGML {
        /// Model ID to load the tokenizer from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        tok_model_id: Option<String>,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(long)]
        tokenizer_json: Option<String>,

        /// Quantized model ID to find the `quantized_filename`.
        /// This may be a HF hub repo or a local path.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Model ID to load LoRA from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        adapters_model_id: String,

        /// Ordering JSON file
        #[arg(short, long)]
        order: String,

        /// GQA value
        #[arg(short, long, default_value_t = 1)]
        gqa: usize,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,
    },

    /// Select a vision plain model, without quantization or adapters
    VisionPlain {
        /// Model ID to load from. This may be a HF hub repo or a local path.
        #[arg(short, long)]
        model_id: String,

        /// Path to local tokenizer.json file. If this is specified it is used over any remote file.
        #[arg(short, long)]
        tokenizer_json: Option<String>,

        /// The architecture of the model.
        #[arg(short, long, value_parser = parse_vision_arch)]
        arch: VisionLoaderType,

        /// Model data type. Defaults to `auto`.
        #[arg(short, long, default_value_t = ModelDType::Auto, value_parser = parse_model_dtype)]
        dtype: ModelDType,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,
    },

    /// Select an EXL2 model
    EXL2 {
        /// `tok_model_id` is the local or remote model ID where you can find a `tokenizer_config.json` file.
        /// If the `chat_template` is specified, then it will be treated as a path and used over remote files,
        /// removing all remote accesses.
        #[arg(short, long)]
        tok_model_id: Option<String>,

        /// Quantized model ID to find the `quantized_filename`.
        /// This may be a HF hub repo or a local path.
        #[arg(short = 'm', long)]
        quantized_model_id: String,

        /// Quantized filename(s).
        /// May be a single filename, or use a delimiter of " " (a single space) for multiple files.
        #[arg(short = 'f', long)]
        quantized_filename: String,

        /// Path to a topology YAML file.
        #[arg(long)]
        topology: Option<String>,

        // Specific EXL2 args
        /// "auto", or VRAM allocation per GPU in GB
        #[arg(short, long)]
        gpu_split: Option<String>,

        /// Maximum sequence length
        #[arg(short, long)]
        length: Option<usize>,

        /// RoPE scaling factor
        #[arg(short, long)]
        rope_scale: Option<f32>,

        /// RoPE alpha value (NTK)
        #[arg(short, long)]
        rope_alpha: Option<f32>,

        /// Disable Flash Attention
        #[arg(long, action)]
        no_flash_attn: bool,

        /// Disable xformers, an alternative plan of flash attn for older devices
        #[arg(long, action)]
        no_xformers: bool,

        /// Disable Torch SDPA
        #[arg(long, action)]
        no_sdpa: bool,

        /// Enable VRAM optimizations, potentially trading off speed
        #[arg(short, long, action)]
        low_mem: bool,

        /// Override MoE model's default number of experts per token
        #[arg(short, long)]
        experts_per_token: Option<usize>,

        /// Load weights in Q4 mode
        #[arg(long, action)]
        load_q4: bool,

        /// Use alternative safetensors loader (with direct I/O when available)
        #[arg(long, action)]
        fast_safetensors: bool,

        /// Do not override model config options in case of compatibility issues
        #[arg(short, long, action)]
        ignore_compatibility: bool,

        /// Chunk size ('input length')
        #[arg(long)]
        chunk_size: Option<usize>,
    },
}
