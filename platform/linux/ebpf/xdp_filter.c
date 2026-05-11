#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 2000000);
    __type(key, __u64);
    __type(value, __u8);
} blocked_domains SEC(".maps");

SEC("xdp")
int xdp_filter(struct xdp_md *ctx) {
    void *data = (void *)(long)ctx->data;
    void *data_end = (void *)(long)ctx->data_end;
    if (data >= data_end) {
        return XDP_PASS;
    }
    return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
