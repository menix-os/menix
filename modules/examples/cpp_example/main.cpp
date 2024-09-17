extern "C"
{
#include <menix/common.h>
#include <menix/drv/module.h>
#include <menix/util/log.h>
}

class Foo
{
  public:
	Foo(const char* msg)
	{
		kmesg(msg);
	}
};

static Foo foo("Hello from C++ constructor!\n");

MODULE_FN i32 init_fn()
{
	kmesg("Hello from C++!\n");
	return 0;
}

MODULE_FN void exit_fn()
{
	return;
}

MODULE_DEFAULT(init_fn, exit_fn);
