use std::alloc::Layout;
use std::ops::Index;

// Stuff to use for interaction with ffi.

fn vec_into_raw_parts<T>(v: Vec<T>) -> (*mut T, usize,usize) {
    let mut me = std::mem::ManuallyDrop::new(v);
    (me.as_mut_ptr(), me.len(), me.capacity())
}

/// A rusty mimic of
/// [`kvm_cpuid`](https://elixir.bootlin.com/linux/v5.10.129/source/arch/x86/include/uapi/asm/kvm.h#L226)
/// .
///
/// [`RawCpuid`] has an identical memory layout to
/// [`kvm_cpuid`](https://elixir.bootlin.com/linux/v5.10.129/source/arch/x86/include/uapi/asm/kvm.h#L226)
/// thus we can [`std::mem::transmute`] between them.
///
/// This allows [`RawCpuid`] to function as a simpler replacement for [`kvm_bindings::CpuId`]. In
/// the future it may replace [`kvm_bindings::CpuId`] fully.
#[derive(Debug)]
#[repr(C)]
pub struct RawCpuid {
    /// Number of entries
    pub nent: u32,
    padding: u32,
    // Pointer to entries
    pub entries: *mut RawCpuidEntry,
}
impl RawCpuid {
    /// Yields an iterator across the entries.
    #[must_use]
    pub fn iter(&'_ self) -> RawCpuidIter<'_> {
        RawCpuidIter {
            cpuid: self,
            count: 0,
        }
    }

    /// Returns an entry for a given lead (function) and sub-leaf (index).
    ///
    /// Returning `None` if it is not present.
    #[must_use]
    pub fn get(&self, leaf: u32, sub_leaf: u32) -> Option<&RawCpuidEntry> {
        self.iter()
            .find(|entry| entry.function == leaf && entry.index == sub_leaf)
    }
}
impl Index<usize> for RawCpuid {
    type Output = RawCpuidEntry;

    /// Indexes across the entries.
    fn index(&self, index: usize) -> &Self::Output {
        assert!(u32::try_from(index).unwrap() < self.nent);
        unsafe { &*self.entries.add(index) }
    }
}
// We implement custom drop which drops all entries using `self.nent`
impl Drop for RawCpuid {
    fn drop(&mut self) {
        unsafe {
            std::alloc::dealloc(
                self.entries.cast::<u8>(),
                Layout::array::<RawCpuidEntry>(self.nent as usize).unwrap(),
            );
        }
    }
}
pub struct RawCpuidIter<'a> {
    cpuid: &'a RawCpuid,
    count: usize,
}
impl<'a> Iterator for RawCpuidIter<'a> {
    type Item = &'a RawCpuidEntry;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.

        let rtn = if self.count == self.cpuid.nent as usize {
            None
        } else {
            Some(unsafe { &*self.cpuid.entries.add(self.count) })
        };
        self.count += 1;
        rtn
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct RawCpuidEntry {
    pub function: u32,
    pub index: u32,
    pub flags: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    padding: [u32; 3],
}
impl RawCpuidEntry {
    #[must_use]
    pub fn new(
        function: u32,
        index: u32,
        flags: u32,
        eax: u32,
        ebx: u32,
        ecx: u32,
        edx: u32,
    ) -> Self {
        Self {
            function,
            index,
            flags,
            eax,
            ebx,
            ecx,
            edx,
            padding: Default::default(),
        }
    }
}
impl From<kvm_bindings::CpuId> for RawCpuid {
    fn from(value: kvm_bindings::CpuId) -> Self {
        // As cannot acquire ownership of the underlying slice, we clone it.
        let cloned = value.as_slice().to_vec();
        let (ptr, len, _cap) = vec_into_raw_parts(cloned);
        Self {
            nent: u32::try_from(len).unwrap(),
            padding: Default::default(),
            entries: ptr.cast::<RawCpuidEntry>(),
        }
    }
}
// We can't implement a foreign trait on a foreign type, thus we can't implement `From<RawCpuid> for
// kvm_bindings::CpuId` thus we must implement `Into`.
#[allow(clippy::from_over_into)]
impl Into<kvm_bindings::CpuId> for RawCpuid {
    fn into(self) -> kvm_bindings::CpuId {
        let cpuid_slice = unsafe { std::slice::from_raw_parts(self.entries, self.nent as usize) };
        // println!("cpuid_slice: {:?}",cpuid_slice);
        #[allow(clippy::transmute_ptr_to_ptr)]
        let kvm_bindings_slice = unsafe { std::mem::transmute(cpuid_slice) };
        kvm_bindings::CpuId::from_entries(kvm_bindings_slice).unwrap()
    }
}
// // We can't implement a foreign trait on a foreign type.
#[allow(clippy::from_over_into)]
impl Into<(u32, u32, u32, u32)> for RawCpuidEntry {
    fn into(self) -> (u32, u32, u32, u32) {
        (self.eax, self.ebx, self.ecx, self.edx)
    }
}

#[cfg(tests)]
mod tests {
    #[cfg(target_os = "linux")]
    #[test]
    fn testing() {
        use kvm_bindings::{kvm_cpuid_entry2, KVM_MAX_CPUID_ENTRIES};

        let kvm = kvm_ioctls::Kvm::new().unwrap();
        let vm = kvm.create_vm().unwrap();
        let vcpu = vm.create_vcpu(0).unwrap();
        let kvm_cpuid = kvm.get_supported_cpuid(KVM_MAX_CPUID_ENTRIES).unwrap();
        check_err();

        println!("kvm_cpuid:");
        for x in kvm_cpuid.clone().as_slice() {
            println!("\t{:?}", x);
        }

        let cpuid = CpuId::from(kvm_cpuid);
        println!("cpuid:");
        for x in cpuid.iter() {
            println!("\t{:?}", x);
        }

        let kvm_cpuid2: kvm_bindings::CpuId = cpuid.into();
        println!("kvm_cpuid2:");
        for x in kvm_cpuid2.clone().as_slice() {
            println!("\t{:?}", x);
        }

        vcpu.set_cpuid2(&kvm_cpuid2).unwrap();
        check_err();

        let kvm_cpuid3 = vcpu.get_cpuid2(KVM_MAX_CPUID_ENTRIES).unwrap();
        check_err();
        println!("kvm_cpuid 3:");
        for x in kvm_cpuid3.as_slice() {
            println!("\t{:?}", x);
        }

        fn check_err() {
            let errno = unsafe { libc::__errno_location() };
            println!("errno: {}", unsafe { *errno });
            let string = std::ffi::CString::new("get_supported_cpuid").unwrap();
            unsafe { libc::perror(string.as_ptr()) };
        }
    }
}
