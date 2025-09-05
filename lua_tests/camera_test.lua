

--[[
local main_camera = camera_main()

print(tostring(main_camera))

main_camera:fov(degf(45))
main_camera:near(0.01)
main_camera:far(100)

print(tostring(main_camera))

local mcg = main_camera:parent()

local c = mcg:get_component("BI_CAMERA")

print(tostring(c))
c("fov", degf(50))
print(tostring(main_camera))]]
