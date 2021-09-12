/// @description draw menu
draw_set_halign(fa_center);
var xm = 160;
var ym = 60;
draw_rectangle(xm-36,ym-8,xm+36,ym+48,1);
for (var i = 0; i < array_length(menu); i++){
	draw_text_color(xm,ym+(16*i)+1,menu[i],c_black,c_black,
	c_black,c_black,0.5);
	draw_set_color(cur_index==i?c_orange:c_white);
	draw_text(xm,ym+(16*i),menu[i]);
}
draw_set_color(c_white);
draw_set_halign(fa_left);