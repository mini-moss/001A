use spin::Once;
use crate::println;

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; 2],
}

impl AppManager {
    fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            println!("All applications completed");
        }
    }
}

static APPMANAGER: Once<AppManager> = Once::new();

pub fn init() {
    APPMANAGER.call_once(|| unsafe {
        unsafe extern "C" {
            fn _num_app();
        }
        let num_app_ptr = _num_app as usize as *const usize;
        let num_app = num_app_ptr.read_volatile();
        let mut app_start: [usize; 2] = [0; 2];
        let app_start_raw: &[usize] = core::slice::from_raw_parts(num_app_ptr.add(1), 2);
        app_start[..=num_app].copy_from_slice(app_start_raw);
        AppManager {
            num_app,
            current_app: 0,
            app_start,
        }
    });
}

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack { data: [0; KERNEL_STACK_SIZE] };
static USER_STACK: UserStack = UserStack { data: [0; USER_STACK_SIZE] };