/* automatically generated by rust-bindgen */
#![allow(non_camel_case_types, non_snake_case)]
use c_types::fd_set;
use c_types::hostent;
use c_types::in_addr;
use c_types::iovec;
use c_types::sockaddr;
use c_types::socklen_t;
use libc::timeval;

#[cfg(target_os = "android")]
use jni_sys;

#[cfg(windows)]
pub type ares_socket_t = ::winapi::um::winsock2::SOCKET;
#[cfg(unix)]
pub type ares_socket_t = ::std::os::unix::io::RawFd;

pub type ares_socklen_t = socklen_t;
pub type ares_ssize_t = isize;
pub type ares_sock_state_cb = ::std::option::Option<
    unsafe extern "C" fn(
        data: *mut ::std::os::raw::c_void,
        socket_fd: ares_socket_t,
        readable: ::std::os::raw::c_int,
        writable: ::std::os::raw::c_int,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct apattern {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ares_options {
    pub flags: ::std::os::raw::c_int,
    pub timeout: ::std::os::raw::c_int,
    pub tries: ::std::os::raw::c_int,
    pub ndots: ::std::os::raw::c_int,
    pub udp_port: ::std::os::raw::c_ushort,
    pub tcp_port: ::std::os::raw::c_ushort,
    pub socket_send_buffer_size: ::std::os::raw::c_int,
    pub socket_receive_buffer_size: ::std::os::raw::c_int,
    pub servers: *mut in_addr,
    pub nservers: ::std::os::raw::c_int,
    pub domains: *mut *mut ::std::os::raw::c_char,
    pub ndomains: ::std::os::raw::c_int,
    pub lookups: *mut ::std::os::raw::c_char,
    pub sock_state_cb: ares_sock_state_cb,
    pub sock_state_cb_data: *mut ::std::os::raw::c_void,
    pub sortlist: *mut apattern,
    pub nsort: ::std::os::raw::c_int,
    pub ednspsz: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ares_channeldata {
    _unused: [u8; 0],
}
pub type ares_channel = *mut ares_channeldata;
pub type ares_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg: *mut ::std::os::raw::c_void,
        status: ::std::os::raw::c_int,
        timeouts: ::std::os::raw::c_int,
        abuf: *mut ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
    ),
>;
pub type ares_host_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg: *mut ::std::os::raw::c_void,
        status: ::std::os::raw::c_int,
        timeouts: ::std::os::raw::c_int,
        hostent: *mut hostent,
    ),
>;
pub type ares_nameinfo_callback = ::std::option::Option<
    unsafe extern "C" fn(
        arg: *mut ::std::os::raw::c_void,
        status: ::std::os::raw::c_int,
        timeouts: ::std::os::raw::c_int,
        node: *mut ::std::os::raw::c_char,
        service: *mut ::std::os::raw::c_char,
    ),
>;
pub type ares_sock_create_callback = ::std::option::Option<
    unsafe extern "C" fn(
        socket_fd: ares_socket_t,
        type_: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type ares_sock_config_callback = ::std::option::Option<
    unsafe extern "C" fn(
        socket_fd: ares_socket_t,
        type_: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn ares_library_init(flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_library_init_mem(
        flags: ::std::os::raw::c_int,
        amalloc: ::std::option::Option<
            unsafe extern "C" fn(size: usize) -> *mut ::std::os::raw::c_void,
        >,
        afree: ::std::option::Option<unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void)>,
        arealloc: ::std::option::Option<
            unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void, size: usize)
                -> *mut ::std::os::raw::c_void,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_library_initialized() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_library_cleanup();
}
extern "C" {
    pub fn ares_version(version: *mut ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn ares_init(channelptr: *mut ares_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_init_options(
        channelptr: *mut ares_channel,
        options: *mut ares_options,
        optmask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_save_options(
        channel: ares_channel,
        options: *mut ares_options,
        optmask: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_destroy_options(options: *mut ares_options);
}
extern "C" {
    pub fn ares_dup(dest: *mut ares_channel, src: ares_channel) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_destroy(channel: ares_channel);
}
extern "C" {
    pub fn ares_cancel(channel: ares_channel);
}
extern "C" {
    pub fn ares_set_local_ip4(channel: ares_channel, local_ip: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn ares_set_local_ip6(channel: ares_channel, local_ip6: *const ::std::os::raw::c_uchar);
}
extern "C" {
    pub fn ares_set_local_dev(channel: ares_channel, local_dev_name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn ares_set_socket_callback(
        channel: ares_channel,
        callback: ares_sock_create_callback,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn ares_set_socket_configure_callback(
        channel: ares_channel,
        callback: ares_sock_config_callback,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn ares_set_sortlist(
        channel: ares_channel,
        sortstr: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ares_socket_functions {
    pub asocket: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: *mut ::std::os::raw::c_void,
        ) -> ares_socket_t,
    >,
    pub aclose: ::std::option::Option<
        unsafe extern "C" fn(arg1: ares_socket_t, arg2: *mut ::std::os::raw::c_void)
            -> ::std::os::raw::c_int,
    >,
    pub aconnect: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ares_socket_t,
            arg2: *const sockaddr,
            arg3: ares_socklen_t,
            arg4: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub arecvfrom: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ares_socket_t,
            arg2: *mut ::std::os::raw::c_void,
            arg3: usize,
            arg4: ::std::os::raw::c_int,
            arg5: *mut sockaddr,
            arg6: *mut ares_socklen_t,
            arg7: *mut ::std::os::raw::c_void,
        ) -> ares_ssize_t,
    >,
    pub asendv: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ares_socket_t,
            arg2: *const iovec,
            arg3: ::std::os::raw::c_int,
            arg4: *mut ::std::os::raw::c_void,
        ) -> ares_ssize_t,
    >,
}
extern "C" {
    pub fn ares_set_socket_functions(
        channel: ares_channel,
        funcs: *const ares_socket_functions,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn ares_send(
        channel: ares_channel,
        qbuf: *const ::std::os::raw::c_uchar,
        qlen: ::std::os::raw::c_int,
        callback: ares_callback,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn ares_query(
        channel: ares_channel,
        name: *const ::std::os::raw::c_char,
        dnsclass: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
        callback: ares_callback,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn ares_search(
        channel: ares_channel,
        name: *const ::std::os::raw::c_char,
        dnsclass: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
        callback: ares_callback,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn ares_gethostbyname(
        channel: ares_channel,
        name: *const ::std::os::raw::c_char,
        family: ::std::os::raw::c_int,
        callback: ares_host_callback,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn ares_gethostbyname_file(
        channel: ares_channel,
        name: *const ::std::os::raw::c_char,
        family: ::std::os::raw::c_int,
        host: *mut *mut hostent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_gethostbyaddr(
        channel: ares_channel,
        addr: *const ::std::os::raw::c_void,
        addrlen: ::std::os::raw::c_int,
        family: ::std::os::raw::c_int,
        callback: ares_host_callback,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn ares_getnameinfo(
        channel: ares_channel,
        sa: *const sockaddr,
        salen: ares_socklen_t,
        flags: ::std::os::raw::c_int,
        callback: ares_nameinfo_callback,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn ares_fds(
        channel: *const ares_channeldata,
        read_fds: *mut fd_set,
        write_fds: *mut fd_set,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_getsock(
        channel: *const ares_channeldata,
        socks: *mut ares_socket_t,
        numsocks: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_timeout(
        channel: ares_channel,
        maxtv: *mut timeval,
        tv: *mut timeval,
    ) -> *mut timeval;
}
extern "C" {
    pub fn ares_process(channel: ares_channel, read_fds: *mut fd_set, write_fds: *mut fd_set);
}
extern "C" {
    pub fn ares_process_fd(channel: ares_channel, read_fd: ares_socket_t, write_fd: ares_socket_t);
}
extern "C" {
    pub fn ares_create_query(
        name: *const ::std::os::raw::c_char,
        dnsclass: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
        id: ::std::os::raw::c_ushort,
        rd: ::std::os::raw::c_int,
        buf: *mut *mut ::std::os::raw::c_uchar,
        buflen: *mut ::std::os::raw::c_int,
        max_udp_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_mkquery(
        name: *const ::std::os::raw::c_char,
        dnsclass: ::std::os::raw::c_int,
        type_: ::std::os::raw::c_int,
        id: ::std::os::raw::c_ushort,
        rd: ::std::os::raw::c_int,
        buf: *mut *mut ::std::os::raw::c_uchar,
        buflen: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_expand_name(
        encoded: *const ::std::os::raw::c_uchar,
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        s: *mut *mut ::std::os::raw::c_char,
        enclen: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_expand_string(
        encoded: *const ::std::os::raw::c_uchar,
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        s: *mut *mut ::std::os::raw::c_uchar,
        enclen: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ares_in6_addr {
    pub _S6_un: ares_in6_addr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ares_in6_addr__bindgen_ty_1 {
    pub _S6_u8: [::std::os::raw::c_uchar; 16usize],
    _bindgen_union_align: [u8; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ares_addrttl {
    pub ipaddr: in_addr,
    pub ttl: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ares_addr6ttl {
    pub ip6addr: ares_in6_addr,
    pub ttl: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ares_srv_reply {
    pub next: *mut ares_srv_reply,
    pub host: *mut ::std::os::raw::c_char,
    pub priority: ::std::os::raw::c_ushort,
    pub weight: ::std::os::raw::c_ushort,
    pub port: ::std::os::raw::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ares_mx_reply {
    pub next: *mut ares_mx_reply,
    pub host: *mut ::std::os::raw::c_char,
    pub priority: ::std::os::raw::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ares_txt_reply {
    pub next: *mut ares_txt_reply,
    pub txt: *mut ::std::os::raw::c_uchar,
    pub length: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ares_txt_ext {
    pub next: *mut ares_txt_ext,
    pub txt: *mut ::std::os::raw::c_uchar,
    pub length: usize,
    pub record_start: ::std::os::raw::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ares_naptr_reply {
    pub next: *mut ares_naptr_reply,
    pub flags: *mut ::std::os::raw::c_uchar,
    pub service: *mut ::std::os::raw::c_uchar,
    pub regexp: *mut ::std::os::raw::c_uchar,
    pub replacement: *mut ::std::os::raw::c_char,
    pub order: ::std::os::raw::c_ushort,
    pub preference: ::std::os::raw::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ares_soa_reply {
    pub nsname: *mut ::std::os::raw::c_char,
    pub hostmaster: *mut ::std::os::raw::c_char,
    pub serial: ::std::os::raw::c_uint,
    pub refresh: ::std::os::raw::c_uint,
    pub retry: ::std::os::raw::c_uint,
    pub expire: ::std::os::raw::c_uint,
    pub minttl: ::std::os::raw::c_uint,
}
extern "C" {
    pub fn ares_parse_a_reply(
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        host: *mut *mut hostent,
        addrttls: *mut ares_addrttl,
        naddrttls: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_parse_aaaa_reply(
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        host: *mut *mut hostent,
        addrttls: *mut ares_addr6ttl,
        naddrttls: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_parse_ptr_reply(
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        addr: *const ::std::os::raw::c_void,
        addrlen: ::std::os::raw::c_int,
        family: ::std::os::raw::c_int,
        host: *mut *mut hostent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_parse_ns_reply(
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        host: *mut *mut hostent,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_parse_srv_reply(
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        srv_out: *mut *mut ares_srv_reply,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_parse_mx_reply(
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        mx_out: *mut *mut ares_mx_reply,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_parse_txt_reply(
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        txt_out: *mut *mut ares_txt_reply,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_parse_txt_reply_ext(
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        txt_out: *mut *mut ares_txt_ext,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_parse_naptr_reply(
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        naptr_out: *mut *mut ares_naptr_reply,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_parse_soa_reply(
        abuf: *const ::std::os::raw::c_uchar,
        alen: ::std::os::raw::c_int,
        soa_out: *mut *mut ares_soa_reply,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_free_string(str: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn ares_free_hostent(host: *mut hostent);
}
extern "C" {
    pub fn ares_free_data(dataptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn ares_strerror(code: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ares_addr_node {
    pub next: *mut ares_addr_node,
    pub family: ::std::os::raw::c_int,
    pub addr: ares_addr_node__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ares_addr_node__bindgen_ty_1 {
    pub addr4: in_addr,
    pub addr6: ares_in6_addr,
    _bindgen_union_align: [u32; 4usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ares_addr_port_node {
    pub next: *mut ares_addr_port_node,
    pub family: ::std::os::raw::c_int,
    pub addr: ares_addr_port_node__bindgen_ty_1,
    pub udp_port: ::std::os::raw::c_int,
    pub tcp_port: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ares_addr_port_node__bindgen_ty_1 {
    pub addr4: in_addr,
    pub addr6: ares_in6_addr,
    _bindgen_union_align: [u32; 4usize],
}
extern "C" {
    pub fn ares_set_servers(
        channel: ares_channel,
        servers: *mut ares_addr_node,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_set_servers_ports(
        channel: ares_channel,
        servers: *mut ares_addr_port_node,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_set_servers_csv(
        channel: ares_channel,
        servers: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_set_servers_ports_csv(
        channel: ares_channel,
        servers: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_get_servers(
        channel: ares_channel,
        servers: *mut *mut ares_addr_node,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_get_servers_ports(
        channel: ares_channel,
        servers: *mut *mut ares_addr_port_node,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ares_inet_ntop(
        af: ::std::os::raw::c_int,
        src: *const ::std::os::raw::c_void,
        dst: *mut ::std::os::raw::c_char,
        size: ares_socklen_t,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn ares_inet_pton(
        af: ::std::os::raw::c_int,
        src: *const ::std::os::raw::c_char,
        dst: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
#[cfg(target_os = "android")]
extern "C" {
    pub fn ares_library_init_jvm(jvm: *mut jni_sys::JavaVM);
}
#[cfg(target_os = "android")]
extern "C" {
    pub fn ares_library_init_android(
        connectivity_manager: jni_sys::jobject,
    ) -> ::std::os::raw::c_int;
}
#[cfg(target_os = "android")]
extern "C" {
    pub fn ares_library_android_initialized() -> ::std::os::raw::c_int;
}
