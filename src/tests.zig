const std = @import("std");
const forestry = @import("root.zig");
const testing = std.testing;

test "basic add functionality" {
    try testing.expect(forestry.add(3, 7) == 10);
}
