#![allow(nonstandard_style)]

include!(concat!(env!("OUT_DIR"), "/vulkan.rs"));

#[inline]
pub fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}