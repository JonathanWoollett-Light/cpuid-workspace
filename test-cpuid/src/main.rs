use kvm_bindings::KVM_MAX_CPUID_ENTRIES;
use test_cpuid::{IntelCpuid, RawCpuid};
// use kvm_bindings::kvm_cpuid_entry2;

fn main() {
    println!("11: {:?}", unsafe {
        core::arch::x86_64::__get_cpuid_max(11)
    });
    println!("11.0: {:?}", unsafe {
        core::arch::x86_64::__cpuid_count(11, 0)
    });
    //     println!("4.1: {:?}", unsafe { core::arch::x86_64::__cpuid_count(4,1) });
    //     println!("4.2: {:?}", unsafe { core::arch::x86_64::__cpuid_count(4,2) });
    //     println!("4.3: {:?}", unsafe { core::arch::x86_64::__cpuid_count(4,3) });
    //     println!("4.4: {:?}", unsafe { core::arch::x86_64::__cpuid_count(4,4) });
    //     println!("4.5: {:?}", unsafe { core::arch::x86_64::__cpuid_count(4,5) });
    //     println!("4.6: {:?}", unsafe { core::arch::x86_64::__cpuid_count(4,6) });
    //     println!("4.7: {:?}", unsafe { core::arch::x86_64::__cpuid_count(4,7) });
    //     println!("4.8: {:?}", unsafe { core::arch::x86_64::__cpuid_count(4,8) });

    let kvm = kvm_ioctls::Kvm::new().unwrap();
    let vm = kvm.create_vm().unwrap();
    let _vcpu = vm.create_vcpu(0).unwrap();
    let kvm_cpuid = kvm.get_supported_cpuid(KVM_MAX_CPUID_ENTRIES).unwrap();
    check_err();
    println!("kvm_cpuid:");
    for x in kvm_cpuid.as_slice() {
        println!("\t{:?}", x);
    }

    let cpuid = RawCpuid::from(kvm_cpuid);
    println!("cpuid:");
    for x in cpuid.iter() {
        println!("\t{:?}", x);
    }
    let intel_cpuid = IntelCpuid::from(cpuid);
    // println!("intel_cpuid: {:#?}", intel_cpuid);

    println!("intel_cpuid.leaf_1.eax: {}", intel_cpuid.leaf_1.eax);
    // println!("intel_cpuid.leaf_1.ebx: {}",intel_cpuid.leaf_1.ebx);
    // println!("intel_cpuid.leaf_1.ecx: {}",intel_cpuid.leaf_1.ecx);
    // println!("intel_cpuid.leaf_1.edx: {}",intel_cpuid.leaf_1.edx);

    // println!("intel_cpuid.leaf_2: {}", intel_cpuid.leaf_2);

    // println!(
    //     "intel_cpuid.leaf4[0]: {} {} {} {}",
    //     intel_cpuid.leaf_4[0].eax,
    //     intel_cpuid.leaf_4[0].ebx,
    //     intel_cpuid.leaf_4[0].ecx,
    //     intel_cpuid.leaf_4[0].edx
    // );
    // println!(
    //     "intel_cpuid.leaf4[1]: {} {} {} {}",
    //     intel_cpuid.leaf_4[1].eax,
    //     intel_cpuid.leaf_4[1].ebx,
    //     intel_cpuid.leaf_4[1].ecx,
    //     intel_cpuid.leaf_4[1].edx
    // );
    // println!(
    //     "intel_cpuid.leaf4[2]: {} {} {} {}",
    //     intel_cpuid.leaf_4[2].eax,
    //     intel_cpuid.leaf_4[2].ebx,
    //     intel_cpuid.leaf_4[2].ecx,
    //     intel_cpuid.leaf_4[2].edx
    // );
    // println!(
    //     "intel_cpuid.leaf4[3]: {} {} {} {}",
    //     intel_cpuid.leaf_4[3].eax,
    //     intel_cpuid.leaf_4[3].ebx,
    //     intel_cpuid.leaf_4[3].ecx,
    //     intel_cpuid.leaf_4[3].edx
    // );

    // println!("intel_cpuid.leaf5: {} {} {} {}",
    // intel_cpuid.leaf5.eax,intel_cpuid.leaf5.ebx,intel_cpuid.leaf5.ecx,intel_cpuid.leaf5.edx);
    // println!(
    //     "intel_cpuid.leaf6: {} {} {} {}",
    //     intel_cpuid.leaf6.eax, intel_cpuid.leaf6.ebx, intel_cpuid.leaf6.ecx,
    // intel_cpuid.leaf6.edx );
    // println!(
    //     "intel_cpuid.leaf7: {} {} {} {}",
    //     intel_cpuid.leaf_7.0.eax,
    //     intel_cpuid.leaf_7.0.ebx,
    //     intel_cpuid.leaf_7.0.ecx,
    //     intel_cpuid.leaf_7.0.edx
    // );
    println!(
        "intel_cpuid.leaf_a: {} {} {} {}",
        intel_cpuid.leaf_a.eax,
        intel_cpuid.leaf_a.ebx,
        intel_cpuid.leaf_a.ecx,
        intel_cpuid.leaf_a.edx
    );

    fn check_err() {
        let errno = unsafe { libc::__errno_location() };
        println!("errno: {}", unsafe { *errno });
        let string = std::ffi::CString::new("get_supported_cpuid").unwrap();
        unsafe { libc::perror(string.as_ptr()) };
    }
}
