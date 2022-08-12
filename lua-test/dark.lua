local handle = io.popen("defaults read -g AppleInterfaceStyle 2>&1")

if handle ~= nil then
  local result = handle:read("*l")
  handle:close()

  if result == "Dark" then
    print("it is dark")
  else
    print("not dark, i.e., light")
  end
end

