/// @description Fullscreen
if(keyboard_check_pressed(vk_f8)){
	window_scale++;
	if(window_scale>max_window_scale){
		window_scale = 1;
	}
	window_set_size(view_width*window_scale,view_height*window_scale);
	surface_resize(application_surface,view_width*window_scale,view_height*window_scale);
	alarm[0] = 1;
}
if keyboard_check_pressed(vk_f9) //Toggle Fullscreen
    {
    if window_get_fullscreen()
       {
       window_set_fullscreen(false);
       }
    else
       {
       window_set_fullscreen(true);
       }
    }