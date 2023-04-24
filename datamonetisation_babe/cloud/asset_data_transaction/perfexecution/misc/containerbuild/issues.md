2022-09-30 08:59:29 👤 Role: AUTHORITY
2022-09-30 08:59:29 💾 Database: RocksDb at /datas/substrate-2/chains/local_testnet/db/full
2022-09-30 08:59:29 ⛓ Native runtime: node-template-100 (node-template-1.tx1.au1)
2022-09-30 08:59:29 Cannot create a runtime error=Other("cannot create module: compilation settings are not compatible with the native host: compilation setting \"has_sse41\" is enabled, but not available on the host")
Error: Service(Client(VersionInvalid("cannot create module: compilation settings are not compatible with the native host: compilation setting \"has_sse41\" is enabled, but not available on the host")))

RUSTFLAGS="-C target-cpu=x86_64" cargo build 


processor       : 63
vendor_id       : GenuineIntel
cpu family      : 15
model           : 6
model name      : Common KVM processor
stepping        : 1
microcode       : 0x1
cpu MHz         : 2299.996
cache size      : 16384 KB
physical id     : 3
siblings        : 16
core id         : 15
cpu cores       : 16
apicid          : 63
initial apicid  : 63
fpu             : yes
fpu_exception   : yes
cpuid level     : 13
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx lm constant_tsc nopl xtopology cpuid tsc_known_freq pni cx16 x2apic hypervisor lahf_lm cpuid_fault pti
bugs            : cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf mds swapgs itlb_multihit
bogomips        : 4599.99
clflush size    : 64
cache_alignment : 128
address sizes   : 40 bits physical, 48 bits virtual

processor       : 52
vendor_id       : GenuineIntel
cpu family      : 6
model           : 6
model name      : QEMU Virtual CPU version 2.5+
stepping        : 3
microcode       : 0x1
cpu MHz         : 2299.998
cache size      : 16384 KB
physical id     : 3
siblings        : 16
core id         : 4
cpu cores       : 16
apicid          : 52
initial apicid  : 52
fpu             : yes
fpu_exception   : yes
cpuid level     : 13
wp              : yes
flags           : fpu de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pse36 clflush mmx fxsr sse sse2 ht syscall nx lm rep_good nopl xtopology cpuid tsc_known_freq pni ssse3 cx16 sse4_1 sse4_2 x2apic hypervisor lahf_lm cpuid_fault pti
bugs            : cpu_meltdown spectre_v1 spectre_v2 spec_store_bypass l1tf mds swapgs itlb_multihit
bogomips        : 4599.99
clflush size    : 64
cache_alignment : 64
address sizes   : 40 bits physical, 48 bits virtual
power management: