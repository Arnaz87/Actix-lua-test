
print("main.lua")

return function ()
  print("handler function")

  -- This should error because myfn expects a string
  -- and is being passed nil, but pcall should catch it.
  -- In the code i'm working on though, it goes over
  -- pcall and panics the underlying rust thread
  print(myfn(nil))
  -- myfn is defined in main.rs:39

  print("after myfn")
end

