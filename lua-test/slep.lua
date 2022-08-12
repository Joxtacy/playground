
local function sleep(sec)
  os.execute("sleep " .. sec)
end

print("sleep")
sleep(1)
print("awake")
