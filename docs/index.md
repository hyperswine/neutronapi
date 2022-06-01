# NeutronAPI

to build a program that targets neutron
just use std and neutron target triple
which builds its std lib

its std lib uses neutronapi to interact with nefs
nefs driver is loaded by default as the root partition
and is binded to neutronapi somehow

so its this 'dynamic' thing that gets me
lets say nefs driver is always statically linked into arcboot and neutron
then you can use its functions like read_block(), read_stuff() directly
or you can use neutronapi. Link neutronapi to neutron and interact with nefs that way
then you can use nefs functions on a suitable partition. You have to load a partition first as a handle
then call neutronapi

nonono. You load the driver. Then call driver.read_to_string()
so its actually the driver that does the stuff
std is merely a way of targeting the syscalls and wrapping around it
the driver does most of its work
how does the driver interact with std? The driver functions are called a syscall to an appropriate file handle
if that file handle refers to an nefs file, it will call nefs_file.read_to_string() or something
neutronapi gives us a consistent way of doing that or something
