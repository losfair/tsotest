# tsotest

Test Total Store Ordering status on your CPU.

## Usage

```bash
cargo install tsotest
tsotest
```

When testing on Macs with the Apple M1 SoC, the `apple-tso` feature is available for switching on the TSO mode. Steps:

1. Build and install the [TSOEnabler for M1](https://github.com/losfair/TSOEnabler) kernel extension.
2. Install with `cargo install tsotest --features apple-tso` .

## Notes

Parallels VMs when running on M1 automatically have TSO mode enabled. I didn't find any public APIs in Hypervisor.framework for reading/writing the `ACTLR_EL1` register (which contains the TSO bit), so it appears that Apple enables it by default.
