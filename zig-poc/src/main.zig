const library = @cImport(
    @cInclude("library.h")
);

const std = @import("std");


pub fn main() !void {
    library.library_function(1, library.HIGH);

    const pair = library.simple_pair {
        .a = 10,
        .b = 20
    };
    library.print_simple_pair(&pair);
}
