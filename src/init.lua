
xpcall(function ()
  local init_f, err = loadfile("main.lua")
  if not init_f then error(err) end

  _G.handler = init_f()
end, function (msg)
  print(msg)
end)