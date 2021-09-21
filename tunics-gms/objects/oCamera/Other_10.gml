/// @description Change Window Scale
var grid = oMenu.menu_pages[oMenu.page];
window_scale = (grid[# 3, oMenu.menu_option[oMenu.page]])+1;
if(window_scale>max_window_scale){
	window_scale = 1;
}
window_set_size(view_width*window_scale,view_height*window_scale);
surface_resize(application_surface,view_width*window_scale,view_height*window_scale);
alarm[0] = 1;