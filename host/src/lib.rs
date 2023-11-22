#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime_interface::runtime_interface;

#[runtime_interface]
pub trait HostApi {
	fn double(n: u32) -> u32 {
		n * 2
	}
}
