// SPDX-License-Identifier: MIT

// TODO: Replace any use of `?` with calls to `.and_then()`.
#[macro_export(local_inner_macros)]
macro_rules! table {
	(
		$lua:ident,
		{ $( $( $key:tt = )? $value:expr ),* $(,)? }
		$( , { $( $( $mt_key:tt = )? $mt_value:expr ),* $(,)? } )?
		$(,)?
	) => {
		{
			let new_table: ::mlua::Table = ::mlua::Lua::create_table($lua)?;
			let mut new_table_key_count: u32 = 0;
			$(new_table.raw_set(($crate::table!(@parse_key, new_table_key_count $( , $key )? )), ($value))?;)*

			$(
				let new_metatable: ::mlua::Table = ::mlua::Lua::create_table($lua)?;
				let mut new_metatable_key_count: u32 = 0;
				$(new_metatable.raw_set(($crate::table!(@parse_key, new_metatable_key_count $( , $mt_key )? )), ($mt_value))?;)*
				new_table.set_metatable(::core::option::Option::Some(new_metatable));
			)?

			::mlua::Result::Ok(new_table)
		}
	};

	(@parse_key, $key_count:ident) => {
		::std::stringify!($key)
	};

	(@parse_key, $key_count:ident, $key:ident) => {
		::std::stringify!($key)
	};

	(@parse_key, $key_count:ident, [ $key:expr ]) => {
		$key
	};
}
