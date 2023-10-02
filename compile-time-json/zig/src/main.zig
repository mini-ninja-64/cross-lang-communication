// const std = @import("std");
// const ComptimeAllocator = @import("comptime_allocator.zig").ComptimeAllocator;

// const DataStructure = struct { key1: []u8, key2: u8, key3: []u8 };

// pub fn parse_json(comptime T: type, comptime input: []const u8) DataStructure {
//     // https://github.com/ziglang/zig/issues/14938
//     var buffer: [1000]u8 = undefined;
//     var allocator = ComptimeAllocator.init(&buffer);
//     if (std.json.parseFromSlice(T, allocator.allocator(), input, .{})) |comptime_json| {
//         return comptime_json;
//     } else |err| {
//         // std.debug.print("{}\n", .{err});
//         @compileError(err);
//     }
// }

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    // std.debug.print("All your {s} are belong to us.\n", .{"codebase"});
    // comptime {
    //     const parsed_str: []u8 = parse_json(DataStructure, @embedFile("simple.json")).key3;
    //     _ = parsed_str;
    // }
    // std.debug.print("{s}\n", .{parsed_json.key3});
}
