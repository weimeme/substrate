// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! 当要执行的 wasm 与本地编译的代码等价时，`CodeExecutor` 的一种特化实现，使用本地编译的运行时。

pub use sc_executor::NativeElseWasmExecutor;

// 声明一个名为 `ExecutorDispatch` 的本地执行器实例。包含与之等价的 wasm 二进制代码作为 wasm 代码。
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	type ExtendHostFunctions = (
		frame_benchmarking::benchmarking::HostFunctions,
		sp_statement_store::runtime_api::HostFunctions,
	);

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		kitchensink_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		kitchensink_runtime::native_version()
	}
}
