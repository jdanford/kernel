//
//  SOS: the Stupid Operating System
//  by Eliza Weisman (eliza@elizas.website)
//
//  Copyright (c) 2015-2017 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! `x86` and `x86_64` control registers
#![warn(missing_docs)]
use core::fmt;

/// `%cr0` contains flags that modify basic processor operation.
pub mod cr0;

/// `%cr4` contains flags that control protected mode execution.
pub mod cr4;

/// A struct bundling together a snapshot of the control registers state.
#[derive(Copy,Clone,Debug)]
pub struct CrState { /// `%cr0` contains flags that control the CPU's operations
                     pub cr0: cr0::Flags
                   , /// `%cr2` contains the page fault linear address
                     pub cr2: usize
                   , /// `%cr3` contains the page table root pointer
                     ///
                     /// TODO: can this be rewritten as a pointer?
                     pub cr3: usize
                   , /// `%cr4` contains flags that control operations in
                     ///  protected mode
                     pub cr4: cr4::Flags
                   }

impl fmt::Display for CrState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!( f, "CR0: {:#08x} CR2: {:#08x} CR3: {:#08x} CR4: {:#08x}"
                , self.cr0, self.cr2, self.cr3, self.cr4)
    }
}

/// Dump the current contents of the control registers to a `CrState`.
pub fn dump() -> CrState {
    let cr0_: usize; let cr2_: usize;
    let cr3_: usize; let cr4_: usize;
    unsafe {
        asm!(  "mov $0, cr0
                mov $1, cr2
                mov $2, cr3
                mov $3, cr4"
            :   "=r"(cr0_)
              , "=r"(cr2_)
              , "=r"(cr3_)
              , "=r"(cr4_)
            ::: "intel"
              , "volatile");
    }
    CrState { cr0: cr0::Flags::from_bits_truncate(cr0_)
            , cr2: cr2_, cr3: cr3_
            , cr4: cr4::Flags::from_bits_truncate(cr4_)
            }

}

/// `$cr2` contains the page fault linear address
pub mod cr2 {

    /// Read the current value from `$cr2`.
    ///
    /// # Safety
    /// + Reading from control registers while not in kernel mode will cause
    ///   a general protection fault.
    pub unsafe fn read() -> usize {
        let result: usize;
        asm!(   "mov $0, cr2"
            :   "=r"(result)
            ::: "intel" );
        result
    }

    /// Write a value to `$cr2`.
    ///
    /// # Safety
    /// + Control registers should generally not be modified during normal
    ///   operation.
    pub unsafe fn write(value: usize) {
        asm!(  "mov cr2, $0"
            :: "r"(value)
            :: "intel");
    }
}

/// `%cr3` contains the page table root pointer
pub mod cr3 {
    use memory::{PAddr, PhysicalPage};

    // #[cfg(target_arch = "x86_64")]
    // use paging::table::{Table, PML4Level};

    /// Read the current value from `$cr3`.
    ///
    /// # Safety
    /// + Reading from control registers while not in kernel mode will cause
    ///   a general protection fault.
    #[cfg(target_arch = "x86_64")]
    pub unsafe fn read() -> PAddr {
        let result: u64;
        asm!(   "mov $0, cr3"
            :   "=r"(result)
            ::: "intel" );
        PAddr::from(result)
    }

    /// Read the current value from `$cr3`.
    ///
    /// # Safety
    /// + Reading from control registers while not in kernel mode will cause
    ///   a general protection fault.
    #[cfg(target_arch = "x86")]
    pub unsafe fn read() -> PAddr {
        let result: u32;
        asm!(   "mov $0, cr3"
            :   "=r"(result)
            ::: "intel" );
        PAddr::from(result)
    }

    /// Write a value to `$cr3`.
    ///
    /// # Safety
    /// + Control registers should generally not be modified during normal
    ///   operation.
    #[cfg(target_arch = "x86_64")]
    pub unsafe fn write(addr: PAddr) {
        let value: u64 = addr.into();
        asm!(  "mov cr3, $0"
            :: "r"(value)
            :  "memory"
            :  "intel");
    }

    /// Write a value to `$cr3`.
    ///
    /// # Safety
    /// + Control registers should generally not be modified during normal
    ///   operation.
    #[cfg(target_arch = "x86")]
    pub unsafe fn write(addr: PAddr) {
        let value: u32 = addr.into();
        asm!(  "mov cr3, $0"
            :: "r"(value)
            :: "intel");
    }

    /// Returns the current Page Directory base frame.
    ///
    /// # Safety
    /// + Reading from control registers while not in kernel mode will cause
    ///   a general protection fault.
    pub unsafe fn current_pagetable_frame() -> PhysicalPage {
        PhysicalPage::containing_addr(read())
    }

    /// Returns the current Page Directory base frame.
    ///
    /// # Safety
    /// + Reading from control registers while not in kernel mode will cause
    ///   a general protection fault.
    #[inline]
    pub unsafe fn set_pagetable_frame(frame: PhysicalPage) {
        write(frame.base_addr())
    }
}
