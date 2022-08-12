local api = vim.api
local buf, win

local function open_window()
  buf = api.nvim_create_buf(false, true)
  api.nvim_buf_set_option(buf, "bufhidden", "wipe")

  -- get dimensions
  local height = api.nvim_buf_get_option("lines")
  local width = api.nvim_buf_get_option("columns")

  -- calculate our floating window size
  local win_height = math.ceil(height * 0.8 - 4)
  local win_width = math.ceil(width * 0.8)

  -- and its starting position
  local row = math.ceil((height - win_height) / 2 - 1)
  local col = math.ceil((width - win_width) / 2)

  -- set some options
  local opts = {
    style = "minimal",
    relative = "editor",
    height = win_height,
    width = win_width,
    row = row,
    col = col,
  }

  local border_opts = {
    style = "minimal",
    relative = "editor",
    height = win_height + 2,
    width = win_width + 2,
    row = row - 1,
    col = col - 1,
  }

  local border_buf = api.nvim_create_buf(false, true)

  local border_lines = { "╔" .. string.rep("═", win_width) .. "╗" }
  local middle_line = "║" .. string.rep(" ", win_width) .. "║"
  for i = 1, win_height do
    table.insert(border_lines, middle_line)
  end
  table.insert(border_lines, "╚" .. string.rep("═", win_width) .. "╝")

  api.nvim_buf_set_lines(border_buf, 0, -1, false, border_lines)
  -- set buffer's (border buf) lines from first line (0) to last (-1)
  -- ignoring out-of-bounds error (false) with lines (border_lines)

  local border_win = api.nvim_open_win(border_buf, true, border_opts)
  -- and finally create it with buffer attached
  win = api.nvim_open_win(buf, true, opts)

  api.nvim_command('au BufWipeout <buffer> exe "silent bwipeout! "' .. border_buf)
end

local function center(str)
  local width = api.nvim_win_get_width(0)
  local shift = math.floor(width / 2) - math.floor(string.len(str) / 2)
  return string.rep(' ', shift) .. str
end

local position = 0

local function update_view(direction)
  position = position + direction
  if position < 0 then position = 0 end -- HEAD~0 is the newest state

  -- we will use vim systemlist function which run shell
  -- command and return result as list
  local result = vim.fn.systemlist('git diff-tree --no-commit-id --name-only -r HEAD~' .. position)

  -- with small indentation results will look better
  for k, v in pairs(result) do
    result[k] = '  ' .. result[k]
  end
  
  api.nvim_buf_set_lines(buf, 0, -1, false, {
    center('What have I done?'),
    center('HEAD~' .. position),
    '',
  })
  api.nvim_buf_set_lines(buf, 3, -1, false, result)

  api.nvim_buf_add_highlight(buf, -1, 'WhidHeader', 0, 0, -1)
  api.nvim_buf_add_highlight(buf, -1, 'WhidSubHeader', 1, 0, -1)

  api.nvim_buf_set_option(buf, 'modifiable', false)
end
