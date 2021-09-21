/// @description Insert description here
global.pause = false;
aspect_ratio = display_get_width()/display_get_height();
view_height = 180;
view_width = round(view_height*aspect_ratio);

if(view_width & 1) view_width++;
if(view_height & 1) view_height++;

max_window_scale = min(floor(display_get_width()/view_width),floor(display_get_height()/view_height));
if(view_height * max_window_scale == display_get_height())
 max_window_scale--;
window_scale = max_window_scale;
window_set_size(view_width*window_scale,view_height*window_scale);

alarm[0]=1;
surface_resize(application_surface,view_width*window_scale,view_height*window_scale);
display_set_gui_size(view_width,view_height);
camera_set_view_size(VIEW,view_width,view_height);
draw_set_font(Font1);