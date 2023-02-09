// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function draw_text_shadow(argument0, argument1, argument2){
	draw_text_color(argument0,argument1-1,argument2,c_black,c_black,c_black,c_black,1);
	draw_text_color(argument0,argument1+1,argument2,c_black,c_black,c_black,c_black,1);
	draw_text_color(argument0+1,argument1,argument2,c_black,c_black,c_black,c_black,1);
	draw_text_color(argument0-1,argument1,argument2,c_black,c_black,c_black,c_black,1);
	draw_text_color(argument0,argument1,argument2,c_white,c_white,c_white,c_white,1);
}