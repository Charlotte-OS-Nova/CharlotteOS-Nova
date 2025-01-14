use core::{arch::x86_64::__cpuid_count, mem::transmute_copy};

use crate::hal::isa::interface::system_info::CpuInfoIfce;

pub enum IsaExtension {
    avx2,
    avx512,
    pml5,
}

pub struct CpuInfo;

impl CpuInfoIfce for CpuInfo {
    type IsaExtension = IsaExtension;
    type Vendor = [u8; 12];
    type Model = [u8; 48];

    fn get_vendor() -> Self::Vendor {
        unsafe { 
            let vendor_string_raw = __cpuid_count(0,0);
            core::mem::transmute::<[u32; 3], [u8; 12]>([vendor_string_raw.ebx, vendor_string_raw.edx, vendor_string_raw.ecx]) 
        }
    }
    fn get_brand() -> Self::Model {
        unsafe {
            let mut brand_string: [u8; 48] = [0; 48];

            let mut cpuid_result = __cpuid_count(0x80000002, 0);
            brand_string[0..4].copy_from_slice(transmute_copy(&cpuid_result.eax));
            brand_string[4..8].copy_from_slice(transmute_copy(&cpuid_result.ebx));
            brand_string[8..12].copy_from_slice(transmute_copy(&cpuid_result.ecx));
            brand_string[12..16].copy_from_slice(transmute_copy(&cpuid_result.edx));

            cpuid_result = __cpuid_count(0x80000003, 0);
            brand_string[16..20].copy_from_slice(transmute_copy(&cpuid_result.eax));
            brand_string[20..24].copy_from_slice(transmute_copy(&cpuid_result.ebx));
            brand_string[24..28].copy_from_slice(transmute_copy(&cpuid_result.ecx));
            brand_string[28..32].copy_from_slice(transmute_copy(&cpuid_result.edx));

            cpuid_result = __cpuid_count(0x80000004, 0);
            brand_string[32..36].copy_from_slice(transmute_copy(&cpuid_result.eax));
            brand_string[36..40].copy_from_slice(transmute_copy(&cpuid_result.ebx));
            brand_string[40..44].copy_from_slice(transmute_copy(&cpuid_result.ecx));
            brand_string[44..48].copy_from_slice(transmute_copy(&cpuid_result.edx));

            brand_string
        }
    }
    fn get_vaddr_sig_bits() -> u8 {
        unsafe {
            let cpuid_result = __cpuid_count(0x80000008, 0);
            cpuid_result.eax as u8
        }
    }
    fn get_paddr_sig_bits() -> u8 {
        unsafe {
            let cpuid_result = __cpuid_count(0x80000008, 0);
            (cpuid_result.eax >> 8) as u8
        }
    }
    fn is_extension_supported(extension: Self::IsaExtension) -> bool {
        match extension {
            IsaExtension::avx2 => {
                unsafe {
                    let cpuid_result = __cpuid_count(7, 0);
                    (cpuid_result.ebx & 0x20) != 0
                }
            }
            IsaExtension::avx512 => {
                unsafe {
                    let cpuid_result = __cpuid_count(7, 0);
                    (cpuid_result.ebx & 0x40000000) != 0
                }
            }
            IsaExtension::pml5 => {
                unsafe {
                    let cpuid_result = __cpuid_count(0x80000008, 0);
                    (cpuid_result.ecx & 0x100) != 0
                }
            }
        }
    }
}