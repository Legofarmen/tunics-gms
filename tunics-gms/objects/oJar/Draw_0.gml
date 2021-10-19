/// @description draw with z
if(state!="destroy"){
	draw_sprite(sShadowSmall,0,x+1,y+(sprite_get_height(sJar)/2))
}
draw_sprite_ext(sprite_index,image_index,x,y-z,1,1,image_angle,c_white,alpha);