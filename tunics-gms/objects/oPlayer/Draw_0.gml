/// @description Draw Cape first, then base
var capeNshirt = asset_get_index(sprite_get_name(sprite_index) + "Color");
draw_sprite_ext(capeNshirt,image_index,x,y,1,1,0,c_aqua,1); //Draw White Cape and Shirt, works beautifully.
draw_self(); //Draw Base
#region Draw Sword
if(state="atk" && localFrame > 0){
	draw_sprite_ext(sPlayerSword,image_index,x,y,1,1,0,c_orange,1);
	if(!instance_exists(oSensor)){
		instance_create_depth(x,y,0,oSensor);
	}else{
		oSensor.sprite_index = sPlayerSword;
		oSensor.image_index = image_index;
	}
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
#region Z Target
if(ztarget == noone){
	if(inputZtarget && instance_exists(oEnemyMole)){
		ztarget = instance_nearest(x,y,oEnemyMole);
		
	}
}else{
	var zw = sprite_get_width(ztarget.sprite_index);
	var zh = sprite_get_height(ztarget.sprite_index);
	draw_sprite_ext(sZtargeting,0,ztarget.x+(zw/2),ztarget.y+(zh/2),-1,1,180,c_white,1); //DownRight
	draw_sprite_ext(sZtargeting,0,ztarget.x-(zw/2),ztarget.y+(zh/2),-1,1,90,c_white,1); //DownLeft
	draw_sprite_ext(sZtargeting,0,ztarget.x+(zw/2),ztarget.y-(zh/2),1,1,0,c_white,1); //UpRight
	draw_sprite_ext(sZtargeting,0,ztarget.x-(zw/2),ztarget.y-(zh/2),1,-1,180,c_white,1); //UpLeft
}
#endregion