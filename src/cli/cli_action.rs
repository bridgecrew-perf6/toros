// SPDX-FileCopyrightText: 2022 Kevin Amado <kamadorueda@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-only

#[derive(Debug)]
pub(crate) enum CliAction {
    Eval { entrypoint: String },
}
