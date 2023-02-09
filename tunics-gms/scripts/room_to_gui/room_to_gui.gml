// Script assets have changed for v2.3.0 see
// https://help.yoyogames.com/hc/en-us/articles/360005277377 for more information
function room_x_to_gui(x, v = 0) {
  return (x - camera_get_view_x(view_camera[v])) * (display_get_gui_width() / camera_get_view_width(view_camera[v]));
}

function room_y_to_gui(y, v = 0) {
  return (y - camera_get_view_y(view_camera[v])) * (display_get_gui_height() / camera_get_view_height(view_camera[v]));
}