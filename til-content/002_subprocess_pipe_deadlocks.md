---
title: Deadlocking Linux subprocesses using pipes
description:
  Recently, I ran into an interesting subprocess deadlock situation on Linux.
  TLDR be aware of pipe buffer sizes!
date: 2024-2-27T19:59:00
author: Thierry Kühni
keywords: linux, subprocess, pipes, rust
published: true
---

In my open-source project [Hive](https://github.com/probe-rs/hive-software),
which is used to test [probe-rs]() against real hardware, there are two main
executables: The monitor and the runner. I won't go into details on the
architecture of Hive, but broadly speaking, the monitor binary acts as a server,
which executes the runner binary as a subprocess during a test run.

I made some updates and improvements to the monitor code, which I later tested
on the hardware soon to realize that the runner binary would run into a deadlock
every time the monitor executed it. I was a bit surprised by this as I hadn't
changed any code in the runner, although the runner contained new probe-rs code
pulled directly from the master branch.

## Backtrace to the rescue

Usually, an easy way to get more info on what is going on in a supposedly
deadlocked process is to acquire its backtrace to get a glance at where it
hangs. So I spun up rust-gdb, attached it to the runner process, and acquired
the following backtrace:

```rust
#0 **GI\_**libc_write (nbytes=137, buf=0x557f416420, fd=1) at ../sysdeps/unix/sysv/linux/write.c:26
#1 **GI\_**libc_write (fd=1, buf=0x557f416420, nbytes=137) at ../sysdeps/unix/sysv/linux/write.c:24
#2 0x00000055621c7528 in std::sys::unix::fd::FileDesc::write () at library/std/src/sys/unix/fd.rs:264
#3 std::sys::unix::stdio::{impl#3}::write () at library/std/src/sys/unix/stdio.rs:43
#4 std::io::Write::write_all<std::sys::unix::stdio::Stdout> () at library/std/src/io/mod.rs:1622
#5 std::io::stdio::{impl#1}::write_all () at library/std/src/io/stdio.rs:142
#6 0x00000055621c7dc0 in std::io::buffered::linewritershim::{impl#1}::write_all<std::io::stdio::StdoutRaw> () at library/std/src/io/buffered/linewritershim.rs:260
#7 std::io::buffered::linewriter::{impl#2}::write_all<std::io::stdio::StdoutRaw> () at library/std/src/io/buffered/linewriter.rs:208
#8 std::io::stdio::{impl#14}::write_all () at library/std/src/io/stdio.rs:747
#9 0x000000556190ca5c in log4rs::priv_io::{impl#2}::write_all (self=0x7fdfc3bcb0, buf=&[u8](size=137) = {...}) at src/priv_io.rs:80
#10 0x0000005561910b6c in log4rs::encode::writer::simple::{impl#0}::write_all<log4rs::priv_io::StdWriterLock> (self=0x7fdfc3bcb0, buf=&[u8](size=137) = {...}) at src/encode/writer/simple.rs:23
#11 0x0000005561908460 in log4rs::append::console::{impl#1}::write_all (self=0x7fdfc3bca8, buf=&[u8](size=137) = {...}) at src/append/console.rs:95
#12 0x00000055618f23ac in controller::logger::encoders::console::{impl#1}::encode (self=0x1, w=..., record=0x7fdfc3c050) at controller/src/logger/encoders/console.rs:45
#13 0x0000005561908610 in log4rs::append::console::{impl#3}::append (self=0x557f402080, record=0x7fdfc3c050) at src/append/console.rs:133
#14 0x0000005561919590 in log4rs::Appender::append (self=0x557f4024e0, record=0x7fdfc3c050) at src/lib.rs:314
#15 0x0000005561919440 in log4rs::ConfiguredLogger::log (self=0x557f4022d0, record=0x7fdfc3c050, appenders=&[log4rs::Appender](size=2) = {...}) at src/lib.rs:284
#16 0x000000556191a24c in log4rs::{impl#5}::log (self=0x557f4021f0, record=0x7fdfc3c050) at src/lib.rs:436
#17 0x000000556210a248 in log::**private_api::log (args=..., level=log::Level::Debug, line=25, kvs=...) at src/**private_api.rs:22
#18 0x0000005561fcda58 in nusb::platform::linux_usbfs::enumeration::SysfsPath::read_attr<alloc::string::String> (self=0x7fdfc3c710, attr="bInterfaceProtocol") at src/platform/linux_usbfs/enumeration.rs:25
--snip--
```

Now, things start to become more apparent. As seen in the backtrace, the runner
hangs at a write syscall that does not seem to return. Walking up the trace, we
can see that this write operation was triggered by
[log4rs](https://github.com/estk/log4rs), which is the logger library I use in
the runner to print the logs to stdout (depending on verbosity) and to a log
file simultaneously. This write is caused by the console appender, which writes
the logs to the stdout. In step 18, we can also see which package is responsible
for the log entry leading to the write syscall. In this case, it's
[nusb](https://github.com/kevinmehall/nusb) (a USB library) which was recently
used in probe-rs to replace [rusb](https://github.com/a1ien/rusb) due to its
dependency on libusb.

> So it was indeed the new probe-rs code!

Well, that was my initial thought, anyway. But it turns out that neither nusb
nor probe-rs are at fault here...

## A deeper look

After following some wrong leads for a while, I had a detailed look at the code
in the monitor, which creates the runner subprocess. Simplified, it looked like
this:

```rust
let mut runner_process = Command::new("path/to/runner")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

let exit_status = runner_process.wait().unwrap();

if !exit_status.success() {
    let mut runner_stdout = vec![];
    runner_process
        .stdout
        .take()
        .unwrap()
        .read_to_end(&mut runner_stdout)
        .unwrap();

    let mut runner_stderr = vec![];
    runner_process
        .stderr
        .take()
        .unwrap()
        .read_to_end(&mut runner_stderr)
        .unwrap();

    println!(
        "Something went wrong: {} {}",
        String::from_utf8(runner_stdout).unwrap(),
        String::from_utf8(runner_stdout).unwrap()
    );
}
```

As can be seen in the code, I create the subprocess using the `Command` struct
from Rust's std-lib and instruct it to send the stdout and stderr back to the
monitor process via pipes (`.stdout(Stdio::piped())`). I then block the thread
and wait for the runner process to exit. If this were unsuccessful, I would
attempt to read the stdout and stderr data of the runner out of the pipes and
print it to the user.

While not the most beautiful code, it looked pretty reasonable to me. After a
break it immediately hit me: The hanging write syscall does not attempt to write
to an actual log file but tries to write to stdout (log4rs console appender),
which in turn is simply a pipe leading to the monitor binary (I know, kinda
obvious in hindsight but I totally overlooked that). So there must be some
problem with the pipe which leads the syscall to block forever (or at least a
long time).

## Pipes in Linux

I now knew the problem was somewhere connected to the Linux pipe used as stdout
and stderr of the runner process. So, it was time to open up the internet. Lo
and behold, the solution was right under my nose:

> Writing more than a pipe buffer’s worth of input to stdin without also reading
> stdout and stderr at the same time may cause a deadlock. This is an issue when
> running any program that doesn’t guarantee that it reads its entire stdin
> before writing more than a pipe buffer’s worth of output. The size of a pipe
> buffer varies on different targets.
> <cite><a href="https://doc.rust-lang.org/std/process/struct.Stdio.html">rust
> std-lib docs</a></cite>

All suddenly made sense: What caused the deadlock was simply the full stdout
pipe buffer. A quick look at `man 7 pipe`
([online](https://man7.org/linux/man-pages/man7/pipe.7.html)) provides some more
insight on the Linux side of things. It is explicitly stated that write attempts
to a full pipe buffer block until enough data has been read from the pipe. The
size of the pipe buffer varies depending on the kernel version. In my case, its
size is 65536 Bytes.

So what lead to the deadlock:

- The change of probe-rs code in the runner binary introduced more log entries
  during the lifecycle of the program
- Thus, log4rs attempted to write more than 65536 Bytes to the stdout pipe
- At the same time, the monitor would not read any data out of the stdio and
  stderr pipe buffers during the execution of the runner

This caused the runner subprocess to become deadlocked by a full pipe buffer as
the monitor would never read any data from the pipes, thus blocking the write
syscall on the runner binary forever.

## Avoiding full pipes

The solution to this problem is simple: We must continuously read from the stdio
and stderr pipes to avoid deadlocking the subprocess. The Rust std-lib does not
provide an out-of-the-box way to do that, so you need to handle this case
yourself. A simple way is to create two additional threads that continuously
read the pipe data into RAM until they receive the EOF condition (you can use
the `read_to_end` Reader trait function for that).

Alternatively, the [rust-subprocess]() library does this for you (even a bit
smarter using `poll` on Linux systems instead of threads). Just make sure to use
the Communicator struct (see
[docs](https://docs.rs/subprocess/latest/subprocess/struct.Communicator.html)).

Overall, this was a great learning experience and a good reminder that it is
crucial to know roughly what the underlying System is doing when developing
software. Given I've spent hours on debugging this, I certainly won't forget
about pipe buffer capacities soon :)
