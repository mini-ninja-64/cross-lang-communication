// https://gist.github.com/rsepassi/d356ea5cfebf37bd9ba8c5d175a7ea30
const std = @import("std");

const Error = std.mem.Allocator.Error;

pub const ComptimeAllocator = struct {
    const Self = @This();

    end_index: usize,
    buffer: []u8,

    /// *WARNING* using this at the same time as the interface returned by `threadSafeAllocator` is not thread safe
    pub fn allocator(self: *ComptimeAllocator) std.mem.Allocator {
        return .{
            .ptr = self,
            .vtable = &.{
                .alloc = alloc,
                .resize = resize,
                .free = free,
            },
        };
    }

    pub fn init(buffer: []u8) ComptimeAllocator {
        return .{
            .buffer = buffer,
            .end_index = 0,
        };
    }

    pub fn alloc(ctx: *anyopaque, len: usize, log2_ptr_align: u8, ret_addr: usize) ?[*]u8 {
        _ = ret_addr;
        const self = @ptrCast(*Self, @alignCast(@alignOf(Self), ctx));
        const ptr_align = @as(usize, 1) << @intCast(std.mem.Allocator.Log2Align, log2_ptr_align);
        const adjust_off = std.mem.alignPointerOffset(self.buffer.ptr + self.end_index, ptr_align) orelse return null;
        const adjusted_index = self.end_index + adjust_off;
        const new_end_index = adjusted_index + len;
        if (new_end_index > self.buffer.len) return null;
        self.end_index = new_end_index;
        return self.buffer.ptr + adjusted_index;
    }

    pub fn resize(ctx: *anyopaque, buf: []u8, buf_align: u8, new_len: usize, ret_addr: usize) bool {
        _ = ret_addr;
        _ = new_len;
        _ = buf_align;
        _ = buf;
        _ = ctx;
        return false;
    }

    pub fn free(ctx: *anyopaque, buf: []u8, buf_align: u8, ret_addr: usize) void {
        _ = ret_addr;
        _ = buf_align;
        _ = buf;
        _ = ctx;
        return;
    }
};
