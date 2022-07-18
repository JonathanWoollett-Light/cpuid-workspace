use kvm_bindings::{kvm_cpuid_entry2, KVM_MAX_CPUID_ENTRIES};
use test_cpuid::RawCpuid;
fn main() {
    let kvm = kvm_ioctls::Kvm::new().unwrap();
    let vm = kvm.create_vm().unwrap();
    let vcpu = vm.create_vcpu(0).unwrap();
    let kvm_cpuid = kvm.get_supported_cpuid(KVM_MAX_CPUID_ENTRIES).unwrap();
    check_err();

    let cpuid = RawCpuid::from(kvm_cpuid);
    println!("cpuid:");
    for x in cpuid.iter() {
        println!("\t{:?}", x);
    }
    fn check_err() {
        let errno = unsafe { libc::__errno_location() };
        println!("errno: {}", unsafe { *errno });
        let string = std::ffi::CString::new("get_supported_cpuid").unwrap();
        unsafe { libc::perror(string.as_ptr()) };
    }
}
        