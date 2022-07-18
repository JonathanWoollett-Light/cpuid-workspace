# test-cpuid

- `RawCpuid`: The low level structure for use with ffi.
- `Cpuid`: The high level structure for general use.

In the future we could use `RawCpuid` as a replacement for `kvm_bindings::CpuId` in rust-vmm. 
At the moment we can simply do `RawCpuid::from(kvm_bindings_cpuid)` when we need to.
Following this we can also convert into `Cpuid` (`Cpuid::from(raw_cpuid)`) when we need to.

For providing a template, one could provide a `.json` which deserializes to `Cpuid` which can then
be converted to `RawCpuid` which can be used to set the cpu cpuid. Without a `Cpuid` which fully 
describes the specification we would need to do some combination with already present features to 
fully form `RawCpuid`.

`Cpuid` could be brought up to date to cover the full AMD and Intel specifications for cpuid, this
while functionally the best approach is a lot of work and would require updating to ensure it
matches the most recent specifications such that Firecracker doesn't become incompatible (or rather 
in this case non-fully compatible) with new hardware. Without this, we will always need to set the
cpuid in the vm by combining our given template with the result of `GET_SUPPORTED_CPUID`.

Printing
```rust
#[test]
fn print() {
    SimpleLogger::new().init().unwrap();
    let cpuid = Cpuid::new();
    println!("cpuid: {:#?}",cpuid);
}
```
```
cpuid: Cpuid {
    leaf0x00_highest_function_parameter_an_manufacturer_id: HighestFunctionParameterAndManufacturerID {
        manufacturer_id: "AuthenticAMD",
        highest_calling_parameter: 13,
    },
    leaf0x01_process_info_and_feature_bits: ProcessorInfoAndFeatureBits {
        processor_version_information: ProcessorVersionInformation {
            stepping_id: 1,
            model: 0,
            family_id: 15,
            processor_type: 0,
            extended_model_id: 6,
            extended_family_id: 8,
        },
        additional_information: AdditionalInformation {
            brand_index: 0,
            clflush_line_size: 8,
            maximum_addressable_logical_processor_ids: 12,
            local_apic_id: 0,
        },
        feature_information: sse3 | pclmulqdq | ssse3 | fma | cx16 | sse4_1 | sse4_2 | movbe | popcnt | aes | xsave | osxsave | avx | f16c | rdrnd | hypervisor | fpu | vme | de | pse | tsc | msr | pae | mce | cx8 | apic | sep | mtrr | pge | mca | cmov | pat | pse_36 | clfsh | mmx | fxsr | sse | sse2 | htt,
    },
    leaf0x06_thermal_and_power_management: ThermalAndPowerManagement {
        features: hardware_coordination_feedback_capability,
        number_of_interrupt_thresholds: 0,
    },
    leaf0x07_extended_features: fsgsbase | bmi1 | avx2 | smep | bmi2 | pqdm | pqe | rdseed | adx | smap | clflushopt | clwb | sha | umip | rdpid,
    cpuid_feature_bitSubLeaf0x1: xsaveopt | xsavec | xgetbv_ecx1 | xss,
    leaf0x12_cpuid_feature_bits: (empty),
    leaf0x14_cpuid_feature_bits: (empty),
    leaf0x19_cpuid_feature_bits: (empty),
    leaf0x80000001_highest_function_parameter_an_manufacturer_id: syscall | nx | mmxext | fxsr_opt | pdpe1gb | rdtscp | lm | 0x183f3ff | lahf | cmp_legacy | cr8_legacy | abm | sse4a | missalignsse | _3dnowprefetch | osvw | wdt | topoext,
    leaf0x80000008_virtual_and_physical_address_sizes: VirtualAndPhysicalAddressSizes {
        number_of_physical_address_bits: 48,
        number_of_linear_address_bits: 48,
        features: clzero | xrstor_fp_err | ibpb | ibrs | single_thread_ibp | virt_ssbd | 0x0x8000000,
        number_of_physical_cores_minus_1: 11,
        log2_of_maximum_apic_id: 7,
        performance_timestamp_counter_size: 0,
    },
    leaf0x8000001f_cpuid_feature_bits: sev_snp | vmpl | restricted_injection | alternative_injection,
}
```

Saving and loading cpuid.
```rust
#[test]
fn serialize_deserialzie() {
    init_logger();
    let cpuid = Cpuid::new();
    println!("cpuid: {:#?}", cpuid);
    let serialized = serde_json::to_string_pretty(&cpuid).unwrap();
    let mut file = File::create("cpuid-x86_64.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
    drop(file);

    let reserialized = read_to_string("cpuid-x86_64.json").unwrap();
    let deserialized: Cpuid = serde_json::from_str(&reserialized).unwrap();
    println!("deserialized: {:#?}", deserialized);
    assert_eq!(cpuid, deserialized);
}
```
Directly accessing registers
```rust
#[test]
fn registers_access() {
    let cpuid = Cpuid::new();

    let _register0_0_eax = cpuid.leaf::<0>().sub_leaf::<0>().eax();
    let _register0_0_ebx = cpuid.leaf::<0>().sub_leaf::<0>().ebx();
    let _register0_0_ecx = cpuid.leaf::<0>().sub_leaf::<0>().ecx();
    let _register0_0_edx = cpuid.leaf::<0>().sub_leaf::<0>().edx();

    let _register1_0_eax = cpuid.leaf::<1>().sub_leaf::<0>().eax();
    let _register1_0_ebx = cpuid.leaf::<1>().sub_leaf::<0>().ebx();
    let _register1_0_ecx = cpuid.leaf::<1>().sub_leaf::<0>().ecx();
    let _register1_0_edx = cpuid.leaf::<1>().sub_leaf::<0>().edx();


    let _register6_0_eax = cpuid.leaf::<6>().sub_leaf::<0>().eax();
    let _register6_0_ebx = cpuid.leaf::<6>().sub_leaf::<0>().ebx();
    let _register6_0_ecx = cpuid.leaf::<6>().sub_leaf::<0>().ecx();


    let _register7_0_ebx = cpuid.leaf::<7>().sub_leaf::<0>().ebx();
    let _register7_0_ecx = cpuid.leaf::<7>().sub_leaf::<0>().ecx();
    let _register7_0_edx = cpuid.leaf::<7>().sub_leaf::<0>().edx();

    let _register7_1_eax = cpuid.leaf::<7>().sub_leaf::<1>().eax();


    let _register_13_1_eax = cpuid.leaf::<13>().sub_leaf::<1>().eax();

    let _register18_0_eax = cpuid.leaf::<18>().sub_leaf::<0>().eax();

    let _register20_0_ebx = cpuid.leaf::<20>().sub_leaf::<0>().ebx();

    let _register0x80000001_0_ecx = cpuid.leaf::<0x80000001>().sub_leaf::<0>().ecx();
    let _register0x80000001_0_edx = cpuid.leaf::<0x80000001>().sub_leaf::<0>().edx();


    let _register0x80000008_0_eax = cpuid.leaf::<0x80000008>().sub_leaf::<0>().eax();
    let _register0x80000008_0_ebx = cpuid.leaf::<0x80000008>().sub_leaf::<0>().ebx();
    let _register0x80000008_0_ecx = cpuid.leaf::<0x80000008>().sub_leaf::<0>().ecx();

    let _register0x8000001f_0_eax = cpuid.leaf::<0x8000001F>().sub_leaf::<0>().eax();
}
```
