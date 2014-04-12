// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

use libc::*;

pub struct ALLEGRO_TRANSFORM
{
	pub m: [[c_float, ..4u], ..4u],
}

extern "C"
{
	pub fn al_use_transform(trans: *ALLEGRO_TRANSFORM);
	pub fn al_copy_transform(dest: *mut ALLEGRO_TRANSFORM, src: *ALLEGRO_TRANSFORM);
	pub fn al_identity_transform(trans: *mut ALLEGRO_TRANSFORM);
	pub fn al_build_transform(trans: *mut ALLEGRO_TRANSFORM, x: c_float, y: c_float, sx: c_float, sy: c_float, theta: c_float);
	pub fn al_translate_transform(trans: *mut ALLEGRO_TRANSFORM, x: c_float, y: c_float);
	pub fn al_rotate_transform(trans: *mut ALLEGRO_TRANSFORM, theta: c_float);
	pub fn al_scale_transform(trans: *mut ALLEGRO_TRANSFORM, sx: c_float, sy: c_float);
	pub fn al_transform_coordinates(trans: *ALLEGRO_TRANSFORM, x: *mut c_float, y: *mut c_float);
	pub fn al_compose_transform(trans: *mut ALLEGRO_TRANSFORM, other: *ALLEGRO_TRANSFORM);
	pub fn al_get_current_transform() -> *ALLEGRO_TRANSFORM;
	pub fn al_invert_transform(trans: *mut ALLEGRO_TRANSFORM);
	pub fn al_check_inverse(trans: *ALLEGRO_TRANSFORM, tol: c_float) -> c_int;
}
