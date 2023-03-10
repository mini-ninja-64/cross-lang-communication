const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    // Standard target options allows the person running `zig build` to choose
    // what target to build for. Here we do not override the defaults, which
    // means any target is allowed, and the default is native. Other options
    // for restricting supported target set are available.
    const target = b.standardTargetOptions(.{});

    // Standard release options allow the person running `zig build` to select
    // between Debug, ReleaseSafe, ReleaseFast, and ReleaseSmall.
    const mode = b.standardReleaseOptions();
    
    const lib_path = "../simple-lib";

    const exe = b.addExecutable("zig-poc", "src/main.zig");

    const cFlags = [_][]const u8{};
    exe.addIncludePath(lib_path ++ "/include");

    // Since zig compiler also supports C we can directly add our C files
    exe.addCSourceFile(lib_path ++ "/src/library.c", &cFlags);
    // Alternatively we can link against binarys that already exist, like so (useful when vendors provide blobs):
    // exe.addLibraryPath(lib_path ++ "/build");
    // exe.linkSystemLibrary("SimpleLib");
    exe.setTarget(target);
    exe.setBuildMode(mode);
    exe.install();

    const run_cmd = exe.run();
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);

    const exe_tests = b.addTest("src/main.zig");
    exe_tests.setTarget(target);
    exe_tests.setBuildMode(mode);

    const test_step = b.step("test", "Run unit tests");
    test_step.dependOn(&exe_tests.step);
}
