#include "rust/cxx.h"

#include <memory>
using std::shared_ptr;
using std::unique_ptr;
using rust::Box;

template <typename T> struct BoxDeleter {
    void operator()(T* ptr){ 
        Box<T> val = Box<T>::from_raw(ptr);
    }
};

template <typename T>
unique_ptr<T, BoxDeleter<T>> box_to_uptr(Box<T>&& box)
{
    return unique_ptr<T, BoxDeleter<T>>(box.into_raw(), BoxDeleter<T>());
}
