local function generate()
  local matches = vim.fn.systemlist("rg TODO -g '!generate.lua'")

  local lines = {}
  table.insert(lines, "# Pending stuff:")
  for i = 1, #matches do
    local match = matches[i]
    local split = vim.fn.split(match, "TODO:")

    if (split[2] ~= nil) then
      local item = "- [ ] " .. split[2]
      table.insert(lines, item)
    end
  end

  vim.fn.writefile(lines, "TODO.md")
end

vim.api.nvim_create_user_command("GenerateTODO", function()
  generate()
end, { nargs = 0 })
