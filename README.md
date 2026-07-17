# the-codex-tools
falls es feheler gibt im code bitte 
discord anschreiben 
dm kk3lly.404
es wurde bisschen mit ki gemacht also bei discord melden falls fehler






# 📖 The Codex

> **A curated grimoire of low-level cybersecurity tools, system-monitoring utilities, and forensic engines written in C# and Rust.**

---

## 🌌 Overview

**The Codex** is a dual-language repository designed for security researchers, system administrators, and curious developers. It bridges the gap between high-level defensive concepts and low-level execution. 

The repository is split into two primary paths:
*   **🛡️ The C# Legation:** Focused on Windows-centric forensics, memory safety, and file integrity.
*   **🦀 The Rust Arcana:** High-performance, zero-copy, and constant-time utilities operating at the system boundary.

---

## 📂 Repository Structure

```text
The-Codex/
├── README.md
├── csharp-codex/          # 🛡️ C# Defensive Utilities
│   ├── EntropyCalculator.cs
│   ├── FileIntegrityMonitor.cs
│   ├── SignatureVerifier.cs
│   ├── SecureMemory.cs
│   └── PrivilegeChecker.cs
└── rust-codex/            # 🦀 Rust System Arcana
    ├── sha256_hasher.rs
    ├── constant_compare.rs







🛡️ C# Codex (The System Shield)These tools leverage Windows APIs and the .NET runtime to inspect, monitor, and shield running systems.File NameTool NameDescriptionKey CyberSec ConceptsEntropyCalculator.csEntropy CalculatorCalculates Shannon Entropy of files to detect packing or encryption.Ransomware Detection, Obfuscation AnalysisFileIntegrityMonitor.csFile Integrity MonitorWatches critical directories (like etc/hosts) in real-time.FIM, Anti-Hijacking, OS HardeningSignatureVerifier.csAuthenticode VerifierProgrammatically validates digital signatures and trust chains.Trust Validation, Code Signing EnforcementSecureMemory.csSecure Memory PinningLocks secrets in physical RAM and zeroes them out after use.RAM-Dump Prevention, Secure CodingPrivilegeChecker.csPrivilege AnalyzerChecks if the current process is running with High Integrity (Admin).Privilege Escalation Defense, UAC


    ├── process_analyzer.rs
    ├── memory_scanner.rs
    └── secure_drop.rs





🦀 Rust Codex (The Performance Rune)Built for speed, safety, and deterministic memory behavior. These tools avoid GC overhead and talk directly to the OS.File NameTool NameDescriptionKey CyberSec Conceptssha256_hasher.rsFast SHA-256 EngineRapidly hashes system binaries using buffered I/O.Forensics, Threat Hunting, IOCsconstant_compare.rsConstant-Time ComparePerforms cryptographic comparisons without early-exit timings.Anti-Side-Channel, Timing Attacksprocess_analyzer.rsLinux /proc ParserReads process arguments directly from kernel state.Hidden Process Detection, Live Forensicsmemory_scanner.rsIn-Memory SignaturesScans RAM-allocated buffers for specific bytecode signatures.YARA-like scanning, Malware Analysissecure_drop.rsRAII Memory ZeroizerAutomatically wipes keys from RAM using Rust's Drop trait.Memory Leak Mitigation, Secrets Management🚀 Getting StartedPrerequisitesC# Tools: .NET SDK (8.0 or newer recommended).Rust Tools: Rustup and the latest stable toolchain.Quick Run: C#Bashcd csharp-codex
dotnet run --project <ToolName>.csproj








⚠️ Disclaimer

    For Educational and Defensive Purposes Only.

    The tools inside The Codex are built strictly to help security teams monitor, audit, and secure systems. Always ensure you have explicit, written authorization before running diagnostic or monitoring software on any target system.
Quick Run: RustBashcd rust-codex
# Compile and run individual binaries
rustc <tool_name>.rs
./<tool_name>
