use core::ptr;

use super::FfaError;

const QUEUE_ENTRY_COUNT: u16 = 0x8;
const QUEUE_BLOCK_SIZE: usize = 0x100;
const QUEUE_STATE_FREE: u8 = 0x0;
const QUEUE_STATE_VALID: u8 = 0x1;

#[repr(C, packed)]
#[derive(Default)]
pub struct AsyncMsgBitmap {
    pub seq_num: u16,
    pub length: u16,
    pub state: u8,
    pub res0: u8,
    pub res1: u16,
}

#[repr(C, packed)]
#[derive(Default)]
pub struct AsyncMsgHeader {
    pub version: u16,
    pub count: u16,
    pub res0: u32,
    pub bitmap: [AsyncMsgBitmap; 15],
}

#[derive(Default)]
pub struct FfaIndirectMsg {}

impl FfaIndirectMsg {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_indirect_msg(&self, base_addr: u64, length: usize) -> FfaError {
        unsafe {
            ptr::write_bytes(base_addr as *mut u8, 0, length);

            let asyncmsg = base_addr as *mut AsyncMsgHeader;
            (*asyncmsg).count = QUEUE_ENTRY_COUNT;
        }
        FfaError::Ok
    }

    /// # Safety
    ///
    /// This function directly reads physical memory pointed to by base_addr
    pub unsafe fn read_indirect_msg(
        &self,
        base_addr: u64,
        seq_num: u16,
        buf: &mut [u8; QUEUE_BLOCK_SIZE],
    ) -> FfaError {
        let asyncmsg = base_addr as *mut AsyncMsgHeader;
        let entry_count = (*asyncmsg).count as usize;
        let mut index = 0;

        for b in &mut (*asyncmsg).bitmap {
            let sn = b.seq_num;
            if sn == seq_num && b.state == QUEUE_STATE_VALID {
                let length = b.length;
                ptr::copy_nonoverlapping(
                    &((*asyncmsg).bitmap[index]) as *const AsyncMsgBitmap as *const u8,
                    buf.as_mut_ptr(),
                    length as usize,
                );
                b.length = 0;
                b.state = QUEUE_STATE_FREE;
                b.seq_num = 0;
                break;
            }
            index += 1;
        }

        if index < entry_count {
            return FfaError::Ok;
        }
        FfaError::Retry
    }

    /// # Safety
    ///
    /// This function directly reads physical memory pointed to by base_addr
    pub unsafe fn write_indirect_msg(&self, base_addr: u64, seq_num: u16, buf: &[u8]) -> FfaError {
        // Note if we have multi-threaded support we need to take a lock while accessing the queue
        let asyncmsg = base_addr as *mut AsyncMsgHeader;
        let entry_count = (*asyncmsg).count as usize;
        let mut data_len = buf.len();
        let mut bm_index: usize = 0;
        let mut data_index: usize = 0;

        for b in &mut (*asyncmsg).bitmap {
            let state = b.state;
            if state == QUEUE_STATE_FREE {
                // Copy data first then update the header structure
                let mut copy_len = QUEUE_BLOCK_SIZE;
                let data_base = base_addr + (QUEUE_BLOCK_SIZE * (bm_index + 1)) as u64;
                if data_len < copy_len {
                    copy_len = data_len;
                }
                ptr::copy_nonoverlapping(&buf[data_index], data_base as *mut u8, copy_len);
                data_index += copy_len;
                data_len -= copy_len;

                b.length = copy_len as u16;
                b.state = QUEUE_STATE_VALID;
                b.seq_num = seq_num;

                // After we've copied over all the data break
                if data_len == 0 {
                    break;
                }
            }
            bm_index += 1;
        }

        if bm_index < entry_count {
            return FfaError::Ok;
        }
        FfaError::NoMemory
    }
}
