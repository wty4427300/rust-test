use std::alloc::{Layout, System};

struct MyAllocator;

//自定义内存分配器
unsafe impl GlobalAlloc for MyAllocator{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let data=System.alloc(layout);
        eprintln!("ALLOC: {:p}, size {}", data, layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        eprintln!("FREE: {:p}, size {}", ptr, layout.size());
    }
}

//设置全局的内存分配器
#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

#[allow(dead_code)]
struct Matrix{
    data:[u8;505],
}

impl Default for Matrix {
    fn default() -> Self {
        Self{
            data:[0;505]
        }
    }
}

fn test(){
    //初始化一个智能指针
    let data=Box::new(Matrix::default());

    eprintln!(
        "!!! allocated memory: {:p}, len: {}",
        &*data,
        std::mem::size_of::<Matrix>());
}