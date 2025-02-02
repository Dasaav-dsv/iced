// SPDX-License-Identifier: MIT
// Copyright (C) 2018-present iced project and contributors

use crate::decoder::handlers::{OpCodeHandler, OpCodeHandlerDecodeFn};
use crate::decoder::table_de::*;
use alloc::vec::Vec;
use lazy_static::lazy_static;

pub(in crate::decoder) struct Tables {
	pub(in crate::decoder) handlers_map0: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(not(feature = "no_vex"))]
	#[allow(dead_code)]
	pub(in crate::decoder) handlers_vex_map0: Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>,
	#[cfg(not(feature = "no_vex"))]
	pub(in crate::decoder) handlers_vex: [Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>; 3],
	#[cfg(not(feature = "no_evex"))]
	pub(in crate::decoder) handlers_evex: [Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>; 6],
	#[cfg(not(feature = "no_xop"))]
	pub(in crate::decoder) handlers_xop: [Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>; 3],
	#[cfg(feature = "mvex")]
	pub(in crate::decoder) handlers_mvex: [Vec<(OpCodeHandlerDecodeFn, &'static OpCodeHandler)>; 3],
	#[cfg(feature = "no_vex")]
	#[allow(dead_code)]
	handlers_vex_map0: (),
	#[cfg(feature = "no_vex")]
	#[allow(dead_code)]
	handlers_vex: [(); 3],
	#[cfg(feature = "no_evex")]
	#[allow(dead_code)]
	handlers_evex: [(); 6],
	#[cfg(feature = "no_xop")]
	#[allow(dead_code)]
	handlers_xop: [(); 3],
	#[cfg(not(feature = "mvex"))]
	#[allow(dead_code)]
	handlers_mvex: [(); 3],
}

lazy_static! {
	pub(in crate::decoder) static ref TABLES: Tables = {
		let handlers_map0 = read_legacy();
		#[cfg(not(feature = "no_vex"))]
		let (handlers_vex_map0, handlers_vex_0f, handlers_vex_0f38, handlers_vex_0f3a) = read_vex();
		#[cfg(not(feature = "no_evex"))]
		let (handlers_evex_0f, handlers_evex_0f38, handlers_evex_0f3a, handlers_evex_map5, handlers_evex_map6) = read_evex();
		#[cfg(not(feature = "no_xop"))]
		let (handlers_xop_map8, handlers_xop_map9, handlers_xop_map10) = read_xop();
		#[cfg(feature = "mvex")]
		let (handlers_mvex_0f, handlers_mvex_0f38, handlers_mvex_0f3a) = read_mvex();
		#[cfg(feature = "no_vex")]
		let (handlers_vex_map0, handlers_vex_0f, handlers_vex_0f38, handlers_vex_0f3a) = ((), (), (), ());
		#[cfg(feature = "no_evex")]
		let (handlers_evex_0f, handlers_evex_0f38, handlers_evex_0f3a, handlers_evex_map5, handlers_evex_map6) = ((), (), (), (), ());
		#[cfg(feature = "no_xop")]
		let (handlers_xop_map8, handlers_xop_map9, handlers_xop_map10) = ((), (), ());
		#[cfg(not(feature = "mvex"))]
		let (handlers_mvex_0f, handlers_mvex_0f38, handlers_mvex_0f3a) = ((), (), ());

		#[cfg(not(feature = "no_evex"))]
		let invalid_map = core::iter::repeat(super::handlers::get_invalid_handler()).take(0x100).collect();
		#[cfg(feature = "no_evex")]
		let invalid_map = ();
		Tables {
			handlers_map0,
			handlers_vex_map0,
			handlers_vex: [handlers_vex_0f, handlers_vex_0f38, handlers_vex_0f3a],
			handlers_evex: [handlers_evex_0f, handlers_evex_0f38, handlers_evex_0f3a, invalid_map, handlers_evex_map5, handlers_evex_map6],
			handlers_xop: [handlers_xop_map8, handlers_xop_map9, handlers_xop_map10],
			handlers_mvex: [handlers_mvex_0f, handlers_mvex_0f38, handlers_mvex_0f3a],
		}
	};
}
