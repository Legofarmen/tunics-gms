/// @description Hitflash
draw_sprite(sprite_index,image_index,x,y-z);
#region Hitflash
if(flash > 0){
    flash -= 0.03;
    shader_set(shFlashColor);
    var shd_alpha = shader_get_uniform(shFlashColor,"_alpha");
	var shd_color = shader_get_uniform(shFlashColor,"_color");
	var col = make_colour_rgb(255,20,0);
    shader_set_uniform_f(shd_alpha, flash);
	shader_set_uniform_f_array(shd_color, [color_get_red(col)/255,color_get_green(col)/255,color_get_blue(col)/255]);
    draw_sprite(sprite_index,image_index,x,y-z);
    shader_reset();
    }
#endregion