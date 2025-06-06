#!/usr/bin/env lua
--- SPDX-License-Identifier: MIT
---@version 5.4
assert(_VERSION == "Lua 5.4")

--------------------------------------------------------------------------------

local dope = require("dope")

print(string.format("dope = %s", tostring(dope)))
for k, v in pairs(dope) do
	print(string.format("dope.%s = %s", tostring(k), tostring(v)))
end
dope.print(nil, true, 0xFFFF, 123.456, "Hello,\
	world! 🤟🏼", {
	"first",
	"second",
	"third",
	foo = "bar",
	biz = "baz",
	["Not an identifier"] = "Yep",
})
