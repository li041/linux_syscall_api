//! 记录该模块使用到的系统调用 id
//!
//!
#[cfg(target_arch = "riscv64")]
numeric_enum_macro::numeric_enum! {
#[repr(usize)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TaskSyscallId {
    EXIT = 93,
    EXIT_GROUP = 94,
    SET_TID_ADDRESS = 96,
    FUTEX = 98,
    SET_ROBUST_LIST = 99,
    GET_ROBUST_LIST = 100,
    NANO_SLEEP = 101,
    GETTIMER = 102,
    SETITIMER = 103,
    CLOCK_GETRES = 114,
    CLOCK_NANOSLEEP = 115,
    SYSLOG = 116,
    SCHED_SETSCHEDULER = 119,
    SCHED_GETSCHEDULER = 120,
    SCHED_SETAFFINITY = 122,
    SCHED_GETAFFINITY = 123,
    SETSID = 157,
    GETRUSAGE = 165,
    UMASK = 166,
    PRCTL = 167,
    GETPID = 172,
    GETPPID = 173,
    GETUID = 174,
    GETEUID = 175,
    GETGID = 176,
    GETEGID = 177,
    GETTID = 178,
    SYSINFO = 179,
    SOCKETPAIR = 199,
    CLONE = 220,
    EXECVE = 221,
    MADVICE = 233,
    WAIT4 = 260,
    GETRANDOM = 278,
    SCHED_YIELD = 124,
    CLOCK_GET_TIME = 113,
    SIGTIMEDWAIT = 137,
    TIMES = 153,
    UNAME = 160,
    GETTIMEOFDAY = 169,
    PRLIMIT64 = 261,
    // 信号模块
    KILL = 129,
    TKILL = 130,
    SIGSUSPEND = 133,
    SIGACTION = 134,
    SIGPROCMASK = 135,
    SIGRETURN = 139,
}
}

#[cfg(target_arch = "x86_64")]
numeric_enum_macro::numeric_enum! {
    #[repr(usize)]
    #[allow(non_camel_case_types)]
    #[allow(missing_docs)]
    #[derive(Eq, PartialEq, Debug, Copy, Clone)]
    pub enum TaskSyscallId {
        ARCH_PRCTL = 158,
        EXIT = 60,
        EXIT_GROUP = 231,
        SET_TID_ADDRESS = 218,
        FUTEX = 202,
        SET_ROBUST_LIST = 273,
        GET_ROBUST_LIST = 274,
        NANO_SLEEP = 35,
        GETTIMER = 36,
        SETITIMER = 38,
        CLOCK_GETRES = 229,
        CLOCK_NANOSLEEP = 230,
        SYSLOG = 103,
        SCHED_SETSCHEDULER = 144,
        SCHED_GETSCHEDULER = 145,
        SCHED_SETAFFINITY = 203,
        SCHED_GETAFFINITY = 204,
        SETSID = 112,
        GETRUSAGE = 98,
        UMASK = 95,
        PRCTL = 157,
        GETPID = 39,
        GETPPID = 110,
        GETUID = 102,
        GETEUID = 107,
        GETGID = 104,
        GETPGID = 121,
        SETPGID = 109,
        GETEGID = 108,
        GETTID = 186,
        SYSINFO = 99,
        SOCKETPAIR = 53,
        CLONE = 56,
        EXECVE = 59,
        MADVICE = 28,
        WAIT4 = 61,
        GETRANDOM = 318,
        SCHED_YIELD = 24,
        CLOCK_GET_TIME = 228,
        SIGTIMEDWAIT = 128,
        TIMES = 100,
        UNAME = 63,
        GETTIMEOFDAY = 96,
        PRLIMIT64 = 302,
        RSEQ = 334,
        // 信号模块
        KILL = 62,
        TKILL = 200,
        SIGSUSPEND = 130,
        SIGACTION = 13,
        SIGPROCMASK = 14,
        SIGRETURN = 15,
        FORK = 57,
        ALARM = 37,
    }
}
