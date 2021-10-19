/// @description draw on screen instead
NineSliceBoxStretched(sprite_index,x1,y1,x2,y2,0);
draw_set_halign(fa_center);
draw_set_valign(fa_top);
var _print = string_copy(text[page],1,textProgress);
var b = c_black;
draw_text_color((x1+x2)/2,y1+8,_print,b,b,b,b,1);
var w = c_white;
draw_text_color((x1+x2)/2,y1+7,_print,w,w,w,w,1);