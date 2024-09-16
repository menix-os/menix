// errno to string conversion

#include <menix/common.h>

#include <errno.h>

const char* strerror(int errno)
{
	switch (errno)
	{
		case E2BIG: return "Argument list too long";
		case EACCES: return "Permission denied";
		case EADDRINUSE: return "Address in use";
		case EADDRNOTAVAIL: return "Address not available";
		case EAFNOSUPPORT: return "Address family not supported";
		case EAGAIN: return "Resource unavailable, try again";
		case EALREADY: return "Connection already in progress";
		case EBADF: return "Bad file descriptor";
		case EBADMSG: return "Bad message";
		case EBUSY: return "Device or resource busy";
		case ECANCELED: return "Operation canceled";
		case ECHILD: return "No child processes";
		case ECONNABORTED: return "Connection aborted";
		case ECONNREFUSED: return "Connection refused";
		case ECONNRESET: return "Connection reset";
		case EDEADLK: return "Resource deadlock would occur";
		case EDESTADDRREQ: return "Destination address required";
		case EDOM: return "Mathematics argument out of domain of function";
		case EDQUOT: return "Reserved";
		case EEXIST: return "File exists";
		case EFAULT: return "Bad address";
		case EFBIG: return "File too large";
		case EHOSTUNREACH: return "Host is unreachable";
		case EIDRM: return "Identifier removed";
		case EILSEQ: return "Illegal byte sequence";
		case EINPROGRESS: return "Operation in progress";
		case EINTR: return "Interrupted function";
		case EINVAL: return "Invalid argument";
		case EIO: return "I/O error";
		case EISCONN: return "Socket is connected";
		case EISDIR: return "Is a directory";
		case ELOOP: return "Too many levels of symbolic links";
		case EMFILE: return "File descriptor value too large";
		case EMLINK: return "Too many links";
		case EMSGSIZE: return "Message too large";
		case EMULTIHOP: return "Reserved";
		case ENAMETOOLONG: return "Filename too long";
		case ENETDOWN: return "Network is down";
		case ENETRESET: return "Connection aborted by network";
		case ENETUNREACH: return "Network unreachable";
		case ENFILE: return "Too many files open in system";
		case ENOBUFS: return "No buffer space available";
		case ENODATA: return "No message is available on the STREAM head read queue";
		case ENODEV: return "No such device";
		case ENOENT: return "No such file or directory";
		case ENOEXEC: return "Executable file format error";
		case ENOLCK: return "No locks available";
		case ENOLINK: return "Reserved";
		case ENOMEM: return "Not enough space";
		case ENOMSG: return "No message of the desired type";
		case ENOPROTOOPT: return "Protocol not available";
		case ENOSPC: return "No space left on device";
		case ENOTCONN: return "The socket is not connected";
		case ENOTDIR: return "Not a directory or a symbolic link to a directory";
		case ENOTEMPTY: return "Directory not empty";
		case ENOTRECOVERABLE: return "State not recoverable";
		case ENOTSOCK: return "Not a socket";
		case ENOTSUP: return "Not supported";
		case ENOTTY: return "Inappropriate I/O control operation";
		case ENXIO: return "No such device or address";
		case EOPNOTSUPP: return "Operation not supported on socket";
		case EOVERFLOW: return "Value too large to be stored in data type";
		case EOWNERDEAD: return "Previous owner died";
		case EPERM: return "Operation not permitted";
		case EPIPE: return "Broken pipe";
		case EPROTO: return "Protocol error";
		case EPROTONOSUPPORT: return "Protocol not supported";
		case EPROTOTYPE: return "Protocol wrong type for socket";
		case ERANGE: return "Result too large";
		case EROFS: return "Read-only file system";
		case ESPIPE: return "Invalid seek";
		case ESRCH: return "No such process";
		case ESTALE: return "Reserved";
		case ETIME: return "Stream ioctl() timeout";
		case ETIMEDOUT: return "Connection timed out";
		case ETXTBSY: return "Text file busy";
		case EXDEV: return "Cross-device link";
	}

	return "Unknown error.";
}
