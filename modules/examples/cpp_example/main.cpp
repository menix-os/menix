extern "C"
{
#include <menix/common.h>
#include <menix/system/module.h>
#include <menix/util/log.h>
}

class Foo
{
  public:
	Foo(const char* msg)
	{
		module_log("%s", msg);
	}
};

static Foo foo("Hello from C++ constructor!\n");

static i32 init_fn()
{
	module_log("Hello from C++!\n");
	return 0;
}

static void exit_fn()
{
	return;
}

MODULE_DEFAULT(init_fn, exit_fn);
