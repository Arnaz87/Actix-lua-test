local res

print("handler.lua")

xpcall(function ()

  -- Returned response
  res = _G.handler(ctx.msg)

end, function (msg)

  print(msg)
  res = msg
  
end)

return res