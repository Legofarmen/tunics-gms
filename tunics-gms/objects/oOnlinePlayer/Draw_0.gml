/// @description Draw Cape first, then base
var capeNshirt = asset_get_index(sprite_get_name(sprite_index) + "Color");
draw_sprite_ext(capeNshirt,image_index,x,y,1,1,0,c_red,1); //Draw White Cape and Shirt, works beautifully.
draw_self(); //Draw Base
#region Draw Sword
if(sprite_index == sPlayerAtk && localFrame > 0){
	draw_sprite_ext(sPlayerSword,image_index,x,y,1,1,0,c_purple,1);
}
#endregion
#region Hitflash
if(flash > 0){
    flash -= 0.03;
    shader_set(shFlashColor);
    var shd_alpha = shader_get_uniform(shFlashColor,"_alpha");
	var shd_color = shader_get_uniform(shFlashColor,"_color");
	var col = make_colour_rgb(255,20,0);
    shader_set_uniform_f(shd_alpha, flash);
	shader_set_uniform_f_array(shd_color, [color_get_red(col)/255,color_get_green(col)/255,color_get_blue(col)/255]);
    draw_self();
    shader_reset();
    }
#endregion
