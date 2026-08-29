# Baseline 001 — Single Node

## Hardware

- CPU: Intel Core i5-9400H
- Cores: 4
- Threads: 8
- RAM: 16 GB
- GPU: Intel UHD Graphics 630
- Architecture: x86_64
- OS: Fedora Linux

## Model

- Model: Qwen2.5-7B-Instruct
- Quantization: Q4_K_M
- Parameters: 7.62B
- Model size: 4.36 GiB
- Backend: CPU
- Threads: 8

## Benchmark

| Test | Performance |
|---|---:|
| Prompt processing (512 tokens) | 19.94 ± 0.08 tok/s |
| Token generation (128 tokens) | 6.30 ± 0.55 tok/s |

## Command

```bash
./vendor/llama.cpp/build/bin/llama-bench \
  -hf Qwen/Qwen2.5-7B-Instruct-GGUF:Q4_K_M \
  -t 8 \
  -p 512 \
  -n 128 \
  -r 3