local M = {}

local theLogic = function(darkFunc, lightFunc)
  local handle = io.popen("defaults read -g AppleInterfaceStyle 2>&1")

  if handle ~= nil then
    local result = handle:read("*l")
    handle:close()

    if result == "Dark" then
      darkFunc()
    else
      lightFunc()
    end

    return result == "Dark"
  end

  return false
end

function M.setBackround()
  local darkFunc = function()
    vim.opt.background = "dark"
  end

  local lightFunc = function()
    vim.opt.background = "light"
  end

  return theLogic(darkFunc, lightFunc)
end

function M.setColorscheme()
  local darkFunc = function()
    vim.cmd("colorscheme nightfox")
  end

  local lightFunc = function()
    vim.cmd("colorscheme dayfox")
  end

  return theLogic(darkFunc, lightFunc)
end

return M
