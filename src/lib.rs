// SPDX-License-Identifier: MIT

mod dope_macros;

use mlua::prelude::*;

#[mlua::lua_module]
fn dope(lua: &Lua) -> LuaResult<LuaTable> {
	table!(lua, {})
}
