local M = {}

function M.sayHelloWorld()
  print("Henlo wurl again!!!")
end

function M.isDark()
  local handle = io.popen("defaults read -g AppleInterfaceStyle 2>&1")

  if handle ~= nil then
    local result = handle:read("*l")
    handle:close()

    if result == "Dark" then
      print("it is dark hurr durr")
      vim.opt.background = "dark"
    else
      print("not dark, i.e., light, muh eyes!")
      vim.opt.background = "light"
    end

    return result == "Dark"
  end

  return false
end

return M
