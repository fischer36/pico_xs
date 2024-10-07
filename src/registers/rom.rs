// Within ROM read-only memory there are a number of functions that are pre-written to every Pico board by default.
// Offsets for the ROM table
const ROM_TABLE_LOOKUP: u16 = 0x00000018;
const ROM_DATA_TABLE: u16 = 0x00000016;
const ROM_FUNC_TABLE: u16 = 0x00000014;

// Function to retrieve function pointer from code in the ROM fn table
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

// Function to convert a 16-bit address to a pointer
unsafe fn rom_hword_as_ptr(rom_address: usize) -> *mut u16 {
    let value = core::ptr::read_volatile(rom_address as *const u16);
    value as usize as *mut u16
}

// Description: Function to perform a lookup in the ROM table.
type RomTableLookupFn = unsafe extern "C" fn(data_table: *mut u16, code: u32) -> *mut u32;

// Functions that are prewritten to every Pico board:

// Fast Bot Counting
// Lookup Code: 'P', '3'
// Description: Counts the number of set bits (1s) in a 32-bit integer.
type PopCount32Fn = unsafe extern "C" fn(value: u32) -> u32;
// Lookup Code: 'R', '3'
// Description: Reverses the bits in a 32-bit integer.
type Reverse32Fn = unsafe extern "C" fn(value: u32) -> u32;
// Lookup Code: 'L', '3'
// Description: Counts the leading zeros in a 32-bit integer.
type Clz32Fn = unsafe extern "C" fn(value: u32) -> u32;
// Lookup Code: 'T', '3'
// Description: Counts the trailing zeros in a 32-bit integer.
type Ctz32Fn = unsafe extern "C" fn(value: u32) -> u32;

// Fast Bulk Memory
// Lookup Code: 'M', 'S'
// Description: Sets `n` bytes starting at `ptr` to the value `c` and returns `ptr`.
type MemsetFn = unsafe extern "C" fn(ptr: *mut u8, value: u8, num: u32) -> *mut u8;
// Lookup Code: 'S', '4'
// Description: A more efficient variant of `_memset` that may only be used if `ptr` is word-aligned.
type Memset4Fn = unsafe extern "C" fn(ptr: *mut u32, value: u8, num: u32) -> *mut u32;
// Lookup Code: 'M', 'C'
// Description: Copies `n` bytes starting at `src` to `dest` and returns `dest`.
// The results are undefined if the regions overlap.
type MemcpyFn = unsafe extern "C" fn(dest: *mut u8, src: *const u8, num: u32) -> *mut u8;
// Lookup Code: 'C', '4'
// Description: A more efficient variant of `_memcpy` that may only be used if `dest` and `src` are word-aligned.
// Copies `n` bytes starting at `src` to `dest` and returns `dest`.
// The results are undefined if the regions overlap.
type Memcpy44Fn = unsafe extern "C" fn(dest: *mut u32, src: *const u32, num: u32) -> *mut u8;

// Flash Access
// Lookup Code: 'I', 'F'
// Description: Restores all QSPI pad controls to their default state and connects the SSI to the QSPI pads.
type ConnectInternalFlashFn = unsafe extern "C" fn();
// Lookup Code: 'E', 'X'
// Description: Sets up the SSI for serial-mode operations and issues the fixed XIP exit sequence.
type FlashExitXipFn = unsafe extern "C" fn();
// Lookup Code: 'R', 'E'
// Description: Erases a range of flash memory.
type FlashRangeEraseFn =
    unsafe extern "C" fn(addr: u32, count: usize, block_size: u32, block_cmd: u8);
// Lookup Code: 'R', 'P'
// Description: Programs data to a range of flash addresses.
type FlashRangeProgramFn = unsafe extern "C" fn(addr: u32, data: *const u8, count: usize);
// Lookup Code: 'F', 'C'
// Description: Flushes and enables the XIP cache.
type FlashFlushCacheFn = unsafe extern "C" fn();
// Lookup Code: 'C', 'X'
// Description: Configures the SSI to generate a standard `03h` serial read command upon each XIP access.
type FlashEnterCmdXipFn = unsafe extern "C" fn();

// Misc
// Lookup Code: 'U', 'B'
pub type ResetToUsbBootFn =
    unsafe extern "C" fn(gpio_activity_mask: u32, disable_interface_mask: u32) -> u32;
// End of file
