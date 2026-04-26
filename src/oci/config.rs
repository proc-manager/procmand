use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::oci::runtime;


// The OCI container config defines the configuration of a container
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OciSpec {
    pub oci_version: String,
    pub root: Option<OciRoot>,
    pub mounts: Option<Vec<OciMount>>,
    pub process: Option<Process>,
    pub hostname: Option<String>,
    pub domainname: Option<String>,
    pub linux: Option<Linux>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OciRoot {
    pub path: PathBuf,
    pub readonly: Option<bool>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OciMount {
    pub destination: PathBuf,
    pub source: Option<PathBuf>,
    pub options: Option<MountOption>, 
    #[serde(rename = "type")]
    pub type_: Option<MountType>,
    pub uid_mappings: Option<Vec<UidMapping>>,
    pub gid_mappings: Option<Vec<GidMapping>>,

}


// TODO: add the other required mount options here
// ref: https://github.com/opencontainers/runtime-spec/blob/main/config.md#linux-mount-options
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MountOption {
    Bind,
    Rbind,
    Ro,
    Rw,
    Noexec,
    Exec,
    Nosuid,
    Suid,
    Nodev,
    Dev,
    Private,
    Rprivate,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MountType {
    Proc,
    Tmpfs,
    Devpts,
    Sysfs,
    Mqueue,
    Cgroup,
    None,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UidMapping {
    pub container_id: u32,
    pub host_id: u32,
    pub size: u32,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GidMapping {
    pub container_id: u32,
    pub host_id: u32,
    pub size: u32,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    pub terminal: Option<bool>,
    pub console_size: Option<ConsoleSize>,
    pub cwd: Option<PathBuf>,
    pub env: Option<Vec<String>>,
    pub args: Option<Vec<String>>,
    pub command_line: Option<String>,
    pub rlimits: Option<Vec<Rlimit>>,
    pub apparmor_profile: Option<String>,
    pub capabilities: Option<Capabilities>,
    pub no_new_privileges: Option<bool>,
    pub oom_score_adj: Option<i64>,
    pub scheduler: Option<Scheduler>,
    pub selinux_label: Option<String>,
    pub io_priority: Option<IoPriority>,

    #[serde(rename = "execCPUAffinity")]
    pub exec_cpu_affinity: Option<ExecCPUAffinity>,
    pub uid: u32,
    pub gid: u32,
    pub umask: u32,
    pub additional_gids: Option<Vec<u32>>
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleSize {
    pub height: u64,
    pub width : u64,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlimit {
    #[serde(rename = "type")]
    pub type_: String,
    pub soft: u64,
    pub hard: u64,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    pub effective: Option<Vec<Capability>>,
    pub bounding: Option<Vec<Capability>>,
    pub inheritable: Option<Vec<Capability>>,
    pub permitted: Option<Vec<Capability>>,
    pub ambient: Option<Vec<Capability>>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Capability {
    CapAuditControl,
    CapAuditRead,
    CapAuditWrite,
    CapBlockSuspend,
    CapBpf,
    CapCheckpointRestore,
    CapChown,
    CapDacOverride,
    CapDacReadSearch,
    CapFowner,
    CapFsetid,
    CapIpcLock,
    CapIpcOwner,
    CapKill,
    CapLease,
    CapLinuxImmutable,
    CapMacAdmin,
    CapMacOverride,
    CapMknod,
    CapNetAdmin,
    CapNetBindService,
    CapNetBroadcast,
    CapNetRaw,
    CapPerfmon,
    CapSetGid,
    CapSetfcap,
    CapSetpcap,
    CapSetUid,
    CapSysAdmin,
    CapSysBoot,
    CapSysChroot,
    CapSysModule,
    CapSysNice,
    CapSysPactt,
    CapSysPtrace,
    CapSysRawio,
    CapSysResource,
    CapSysTime,
    CapSysTtyConfig,
    CapSysLog,
    CapWakeAlarm,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scheduler {
    pub policy: SchedulerPolicy,
    pub nice: Option<i32>,
    pub priority: Option<i32>,
    pub flags: Option<Vec<SchedulerFlags>>,
    pub runtime: Option<u64>,
    pub deadline: Option<u64>,
    pub period: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SchedulerPolicy {
    SchedOther,
    SchedFifo,
    SchedRr,
    SchedBatch,
    SchedIso,
    SchedIdle,
    SchedDeadline
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SchedulerFlags {
    SchedFlagResetOnFork,
    SchedFlagReclaim,
    SchedFlagDlOverrun,
    SchedFlagKeepPolicy,
    SchedFlagKeepParams,
    SchedFlagUtilCapMin,
    SchedFlagUtilCapMax,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoPriority {
    pub class: IoPriorityClass,
    pub priority: u8
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IoPriorityClass {
    IoprioClassRt,
    IoprioClassBe,
    IoprioClassIdle,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecCPUAffinity {
    pub initial: Option<String>,
    #[serde(rename = "final")]
    pub final_: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Linux {
    pub namespaces: Vec<LinuxNamespace>,
    pub uid_mappings: Option<Vec<UidMapping>>,
    pub gid_mappings: Option<Vec<GidMapping>>,
    pub timeoffsets: Option<LinuxTimeOffset>,
    pub devices: Option<LinuxDevice>,
    pub net_devices: Option<HashMap<String, LinuxNetDevice>>,
    pub cgroups_path: Option<PathBuf>,
    pub resources: Option<LinuxResources>,
    pub unified: Option<HashMap<String, String>>,
    pub intel_rdt: Option<LinuxIntelRdt>,
    pub memory_policy: Option<LinuxMemoryPolicy>,
    pub sysctl: Option<HashMap<String, String>>,
    pub seccomp: Option<LinuxSeccomp>,
    pub rootfs_propagation: Option<LinuxRootFsPropagation>,
    pub masked_paths: Option<Vec<PathBuf>>,
    pub readonly_paths: Option<Vec<PathBuf>>,
    pub mount_label: Option<String>,
    pub personality: Option<LinuxPersonality>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxNamespace {
    #[serde(rename = "type")]
    pub type_: LinuxNamespaceType,
    pub path: Option<PathBuf>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LinuxNamespaceType {
    Pid,
    Network,
    Mount,
    Ipc,
    Uts,
    User,
    Cgroup,
    Time
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxTimeOffset {
    pub secs: Option<i64>,
    pub nanosecs: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxDevice {
    #[serde(rename = "type")]
    pub type_: LinuxDeviceType,
    pub path: PathBuf,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub file_mode: Option<u32>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LinuxDeviceType {
    C,
    B,
    U,
    P,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxNetDevice {
    pub name: Option<String>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResources {
    pub devices: Option<Vec<LinuxResourceDevice>>,
    pub memory: Option<LinuxResourceMemory>,
    pub cpu: Option<LinuxResourceCPU>,
    pub block_io: Option<LinuxResourceBlockIO>,
    pub hugepage_limits: Option<Vec<LinuxResourceHugePageLimit>>,
    pub network: Option<LinuxResourceBlockIONetwork>,
    pub pids: Option<LinuxResourcePid>,
    pub rdma: Option<HashMap<String, LinuxResourceRdma>>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourceDevice {
    pub allow: bool,
    #[serde(rename = "type")]
    pub type_: LinuxDeviceType,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub access: Option<String>,
}



#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourceMemory {
    pub limit: Option<i64>,
    pub reservation: Option<i64>,
    pub swap: Option<i64>,
    pub kernel: Option<i64>,
    pub kernel_tcp: Option<i64>,
    pub swappiness: Option<u64>,
    #[serde(rename = "disableOOMKiller")]
    pub disable_oom_killer: Option<bool>,
    pub user_hierarchy: Option<bool>,
    pub check_before_update: Option<bool>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourceCPU {
    pub shares: Option<u64>,
    pub quota: Option<i64>,
    pub burst: Option<u64>,
    pub period: Option<u64>,
    pub realtime_runtime: Option<i64>,
    pub realtime_period: Option<u64>,
    pub cpus: Option<String>,
    pub mems: Option<String>,
    pub idle: Option<i64>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourceBlockIO {
    pub weight: Option<u16>,
    pub leaf_weight: Option<u16>,
    pub weight_device: Option<Vec<LinuxResourceBlockIOWeightDevice>>,
    pub throttle_read_bps_device: Option<Vec<LinuxResourceBlockIOThrottleDevice>>,
    pub throttle_write_bps_device: Option<Vec<LinuxResourceBlockIOThrottleDevice>>,
    pub throttle_read_iops_device: Option<Vec<LinuxResourceBlockIOThrottleDevice>>,
    pub throttle_write_iops_device: Option<Vec<LinuxResourceBlockIOThrottleDevice>>,

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourceBlockIOWeightDevice {
    pub major: i64,
    pub minor: i64,
    pub weight: u16,
    pub leaf_weight: u16,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourceBlockIOThrottleDevice {
    pub major: i64,
    pub minor: i64, 
    pub rate: u64,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourceHugePageLimit {
    pub page_size: String,
    pub limit: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourceBlockIONetwork {
    pub class_id: Option<u32>,
    pub priorities: Option<Vec<LinuxResourceHugePageLimit>>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourceBlockIONetworkPriority {
    pub name: String,
    pub priority: u32
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourcePid {
    pub limits: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxResourceRdma{
    pub hca_handles: Option<u32>,
    pub hca_objects: Option<u32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxIntelRdt {
    pub clos_id: Option<String>,
    pub l3_cache_schema: Option<String>,
    pub mem_bw_schema: Option<String>,
    pub schemata: Option<Vec<String>>,
    pub enable_monitoring: Option<bool>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxMemoryPolicy {
    pub mode: LinuxMemoryPolicyMode,
    pub nodes: Option<String>,
    pub flags: Option<Vec<LinuxMemoryPolicyFlags>>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LinuxMemoryPolicyMode {
    MpolDefault,
    MpolBind,
    MpolInterleave,
    MpolWeightedInterleave,
    MpolPreferredMany,
    MpolLocal,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LinuxMemoryPolicyFlags {
    MpolFNumaBalancing,
    MpolFRelativeNodes,
    MpolFStaticNodes,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxSeccomp {
    pub default_action: String,
    pub default_errno_ret: Option<u16>,
    pub architectures: Option<Vec<LinuxSeccompArch>>,
    pub flags: Option<Vec<LinuxSeccompFlags>>,
    pub listener_path: Option<String>,
    pub listener_metadata: Option<String>,
    pub syscalls: Option<Vec<LinuxSeccompSyscall>>,
    pub oci_version: String,
    pub fds: Option<Vec<String>>,
    pub pid: u8,
    pub metadata: Option<String>,
    pub state: Option<runtime::ContainerState>,
}

#[derive(Serialize, Deserialize)]
pub enum LinuxSeccompArch {
    #[serde(rename = "SCMP_ARCH_X86")]
    ScmpArchX86,
    #[serde(rename = "SCMP_ARCH_X86_64")]
    ScmpArchX8664,
    #[serde(rename = "SCMP_ARCH_X32")]
    ScmpArchX32,
    #[serde(rename = "SCMP_ARCH_ARM")]
    ScmpArchArm,
    #[serde(rename = "SCMP_ARCH_AARCH64")]
    ScmpArchAarch64,
    #[serde(rename = "SCMP_ARCH_MIPS")]
    ScmpArchMips,
    #[serde(rename = "SCMP_ARCH_MIPS64")]
    ScmpArchMips64,
    #[serde(rename = "SCMP_ARCH_MIPS64N32")]
    ScmpArchMips64N32,
    #[serde(rename = "SCMP_ARCH_MIPSEL")]
    ScmpArchMipsel,
    #[serde(rename = "SCMP_ARCH_MIPSEL64")]
    ScmpArchMipsel64,
    #[serde(rename = "SCMP_ARCH_MIPSEL64N32")]
    ScmpArchMipsel64N32,
    #[serde(rename = "SCMP_ARCH_PPC")]
    ScmpArchPpc,
    #[serde(rename = "SCMP_ARCH_PPC64")]
    ScmpArchPpc64,
    #[serde(rename = "SCMP_ARCH_PPC64LE")]
    ScmpArchPpc64Le,
    #[serde(rename = "SCMP_ARCH_S390")]
    ScmpArchS390,
    #[serde(rename = "SCMP_ARCH_S390X")]
    ScmpArchS390X,
    #[serde(rename = "SCMP_ARCH_PARISC")]
    ScmpArchParisc,
    #[serde(rename = "SCMP_ARCH_PARISC64")]
    ScmpArchParisc64,
    #[serde(rename = "SCMP_ARCH_RISCV64")]
    ScmpArchRiscv64,
    #[serde(rename = "SCMP_ARCH_LOONGARCH64")]
    ScmpArchLoongarch64,
    #[serde(rename = "SCMP_ARCH_M68K")]
    ScmpArchM68K,
    #[serde(rename = "SCMP_ARCH_SH")]
    ScmpArchSh,
    #[serde(rename = "SCMP_ARCH_SHEB")]
    ScmpArchSheb,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LinuxSeccompFlags {
    SeccompFilterFlagTsync,
    SeccompFilterFlagLog,
    SeccompFilterFlagSpecAllow,
    SeccompFilterFlagWaitKillableRecv,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxSeccompSyscall {
    pub names: Vec<String>,
    pub action: LinuxSeccompSyscallAction,
    pub errno_ret: Option<u16>,
    pub args: Option<Vec<LinuxSeccompSyscallArg>>
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LinuxSeccompSyscallAction {
    ScmpActKill,
    ScmpActKillProcess,
    ScmpActKillThread,
    ScmpActTrap,
    ScmpActErrno,
    ScmpActTrace,
    ScmpActAllow,
    ScmpActLog,
    ScmpActNotify
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LinuxSeccompSyscallArg {
    ScmpCmpNe,
    ScmpCmpLT,
    ScmpCmpLe,
    ScmpCmpEq,
    ScmpCmpGe,
    ScmpCmpGt,
    ScmpCmpMaskedEq,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LinuxRootFsPropagation {
    Shared,
    Slave,
    Private,
    Unbindable,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxPersonality {
    pub domain: LinuxPersonalityDomain,
    pub flags: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LinuxPersonalityDomain {
    LINUX,
    LINUX32
}
