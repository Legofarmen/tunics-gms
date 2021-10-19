// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function NineSliceBoxStretched(_sprite, _x1, _y1, _x2, _y2, _index) {	
		var _size = sprite_get_width(_sprite) / 3;
		var _w = _x2 - _x1;
		var _h = _y2 - _y1

		//MIDDLE
		draw_sprite_part_ext(_sprite, _index, _size, _size, 1, 1, _x1 + _size,_y1 + _size, _w - (_size * 2), _h - (_size * 2), c_white,1);

		//CORNERS
		//TOP LEFT
		draw_sprite_part(_sprite, _index, 0, 0, _size, _size, _x1, _y1);
		//TOP RIGHT
		draw_sprite_part(_sprite, _index, _size * 2, 0, _size, _size, _x1 + _w - _size, _y1);
		//BOTTOM LEFT
		draw_sprite_part(_sprite, _index, 0, _size * 2, _size, _size, _x1, _y1 + _h - _size);
		//BOTTOM RIGHT
		draw_sprite_part(_sprite, _index, _size * 2, _size * 2, _size, _size, _x1 + _w - _size, _y1 + _h - _size);

		//EDGES
		//LEFT
		draw_sprite_part_ext(_sprite, _index, 0, _size, _size, 1, _x1, _y1 + _size, 1, _h - (_size * 2), c_white, 1);
		//RIGHT
		draw_sprite_part_ext(_sprite, _index, _size * 2, _size, _size, 1, _x1 + _w - _size, _y1 + _size, 1, _h - (_size * 2), c_white, 1);
		//TOP
		draw_sprite_part_ext(_sprite, _index, _size, 0, 1, _size, _x1 + _size, _y1, _w - (_size * 2), 1, c_white, 1);
		//BOTTOM
		draw_sprite_part_ext(_sprite, _index, _size, _size * 2, 1, _size, _x1 + _size, _y1 + _h - (_size), _w - (_size * 2), 1, c_white,1);
	
	}