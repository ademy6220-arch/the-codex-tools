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
