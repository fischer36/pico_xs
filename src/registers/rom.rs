/// Addresses for the ROM table entries.
const ROM_DATA_TABLE: u16 = 0x00000016;
const ROM_FUNC_TABLE: u16 = 0x00000014;
const ROM_TABLE_LOOKUP: u16 = 0x00000018;

// Function to retrieve function pointer from code in the ROM fn table
type RomTableLookupFn = unsafe extern "C" fn(table: *const u16, code: u32) -> *const ();

/// Retrieves a function pointer from the ROM function table based on a two-character code.
///
/// # Arguments
///
/// * `c1` - The first character of the function code.
/// * `c2` - The second character of the function code.
///
/// # Returns
///
/// *mut u32 - 32-bit pointer.
///
/// # Safety
///
/// This function performs raw pointer dereferencing and assumes that the addresses
/// of the ROM functions are correct. It must only be called in an environment where
/// such access is safe.
pub fn get_fn(c1: char, c2: char) -> *mut u32 {
    unsafe {
        let p_rom_table_lookup = ROM_TABLE_LOOKUP as *const usize;
        let func_addr = core::ptr::read_volatile(p_rom_table_lookup);

        // Step 2: Cast the address to a function pointer
        let rom_table_lookup: RomTableLookupFn = core::mem::transmute(func_addr);

        // Step 3: Obtain pointers to the data and function tables
        let data_table: *mut u16 = rom_hword_as_ptr(ROM_DATA_TABLE as usize);
        let func_table: *mut u16 = rom_hword_as_ptr(ROM_FUNC_TABLE as usize);

        // Step 4: Generate code for given characters that identifies the function
        let code = ((c2 as u32) << 8) | (c1 as u32);
        // Step 5: Return the function ptr for given code

        return rom_table_lookup(func_table, code);
    }
}

// !TODO Helper function, might not be needed need to check
unsafe fn rom_hword_as_ptr(rom_address: usize) -> *mut u16 {
    let value = core::ptr::read_volatile(rom_address as *const u16);
    value as usize as *mut u16
}

// Function types for the ROM functions.

/// Counts the number of set bits (1s) in a 32-bit integer.
/// Lookup Code: 'P', '3'
pub type PopCount32Fn = unsafe extern "C" fn(value: u32) -> u32;

/// Reverses the bits in a 32-bit integer.
/// Lookup Code: 'R', '3'
pub type Reverse32Fn = unsafe extern "C" fn(value: u32) -> u32;

/// Counts the leading zeros in a 32-bit integer.
/// Lookup Code: 'L', '3'
pub type Clz32Fn = unsafe extern "C" fn(value: u32) -> u32;

/// Counts the trailing zeros in a 32-bit integer.
/// Lookup Code: 'T', '3'
pub type Ctz32Fn = unsafe extern "C" fn(value: u32) -> u32;

/// Sets `n` bytes starting at `ptr` to the value `c` and returns `ptr`.
/// Lookup Code: 'M', 'S'
pub type MemsetFn = unsafe extern "C" fn(ptr: *mut u8, value: u8, num: u32) -> *mut u8;

/// A more efficient variant of `memset` for word-aligned pointers.
/// Lookup Code: 'S', '4'
pub type Memset4Fn = unsafe extern "C" fn(ptr: *mut u32, value: u8, num: u32) -> *mut u32;

/// Copies `n` bytes from `src` to `dest` and returns `dest`.
/// The results are undefined if the regions overlap.
/// Lookup Code: 'M', 'C'
pub type MemcpyFn = unsafe extern "C" fn(dest: *mut u8, src: *const u8, num: u32) -> *mut u8;

/// A more efficient variant of `memcpy` for word-aligned pointers.
/// Lookup Code: 'C', '4'
pub type Memcpy44Fn = unsafe extern "C" fn(dest: *mut u32, src: *const u32, num: u32) -> *mut u8;

/// Restores all QSPI pad controls to their default state and connects the SSI to the QSPI pads.
/// Lookup Code: 'I', 'F'
pub type ConnectInternalFlashFn = unsafe extern "C" fn();

/// Sets up the SSI for serial-mode operations and issues the fixed XIP exit sequence.
/// Lookup Code: 'E', 'X'
pub type FlashExitXipFn = unsafe extern "C" fn();

/// Erases a range of flash memory.
/// Lookup Code: 'R', 'E'
pub type FlashRangeEraseFn = unsafe extern "C" fn(addr: u32, count: usize, block_size: u32, block_cmd: u8);

/// Programs data to a range of flash addresses.
/// Lookup Code: 'R', 'P'
pub type FlashRangeProgramFn = unsafe extern "C" fn(addr: u32, data: *const u8, count: usize);

/// Flushes and enables the XIP cache.
/// Lookup Code: 'F', 'C'
pub type FlashFlushCacheFn = unsafe extern "C" fn();

/// Configures the SSI to generate a standard `03h` serial read command upon each XIP access.
/// Lookup Code: 'C', 'X'
pub type FlashEnterCmdXipFn = unsafe extern "C" fn();

/// Resets to USB boot.
/// Lookup Code: 'U', 'B'
pub type ResetToUsbBootFn = unsafe extern "C" fn(gpio_activity_mask: u32, disable_interface_mask: u32) -> u32;
