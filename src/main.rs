#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;
use bootloader::{BootInfo, entry_point};

// static HELLO: &[u8] = b"Hello World!";

entry_point!(kernel_main);

// #[no_mangle] // don't mangle the name of this function
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    // vga_buffer::print_something();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    // use blog_os::memory::active_level_4_table;
    // use blog_os::memory::translate_addr;
    // use x86_64::VirtAddr;

    use blog_os::memory;
    use blog_os::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Translate, structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    // panic!("Some panic message");
    
    blog_os::init();

    // let ptr = 0xdeadbeaf as *mut u32;
    // unsafe { *ptr = 42; }
    
    // let ptr = 0x2053e3 as *mut u32;
    // unsafe { let _x = *ptr; }
    // println!("read worked");
    
    // unsafe { *ptr = 42; }
    // println!("write worked");

    // use x86_64::registers::control::Cr3;
    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table = unsafe {
    //     active_level_4_table(phys_mem_offset)
    // };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);
    //     }
    // }

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let addresses = [
    //     0xb8000,
    //     0x201008,
    //     0x0100_0020_1a10,
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = unsafe {translate_addr(virt, phys_mem_offset)};
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {memory::init(phys_mem_offset)};
    // let mut frame_allocator = memory::EmptyFrameAllocator;
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // let page = Page::containing_address(VirtAddr::new(0));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // let addresses = [
    //     0xb8000,
    //     0x201008,
    //     0x0100_0020_1a10,
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }
    
    // x86_64::instructions::interrupts::int3();

    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // }
    
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    //
    // stack_overflow();
    
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    // loop {}
    // loop {
    //     // use blog_os::print;
    //     // for _i in 0..10000 {}
    //     // print!("-");
    // }
    blog_os::hlt_loop();
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info);
}
