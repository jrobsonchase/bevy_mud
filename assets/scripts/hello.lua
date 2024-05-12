local module = {}

local i = 1

local function hello()
  print("Hello, world!")
end

local function count(threadid)
  -- print("i: " .. tostring(i) .. "\tthread: " .. threadid)
  i = i + 2
  return i
end

function module.something_else(threadid)
  return count(threadid)
end

function module.update_self(threadid)
  return count(threadid.." (self)")
end

local Foo = WORLD:component("hello::Foo")
print("FOO: "..tostring(Foo))
local Spam = WORLD:component("hello::Spam")
print("SPAM: "..tostring(Spam))

local newline = string.byte("\n", 1)

local function lines(s)
  return function()
    if s == nil then
      return nil
    end

    for i=1,string.len(s) do
      if string.byte(s, i) == newline then
        local line = string.char(string.byte(s, 1, i-1))
        s = string.char(string.byte(s, i+1, string.len(s)))
        return line
      end
    end

    local line = s
    s = nil

    return line
  end
end


local function print_table(t)
  local out = "{\n"
  local inner = ""
  for k, v in pairs(t) do
    inner = inner..tostring(k)..": "
    if type(v) == "table" then
      inner = inner..print_table(v)
    else
      inner = inner..tostring(v)
    end
    inner = inner..",\n"
  end
  for l in lines(inner) do
    if l ~= "" then
      out = out.."  "..l.."\n"
    end
  end
  out = out.."}"
  return out
end

local query = WORLD:query():with(Foo):build()
function module.update_global(threadid)
  -- for _, ent in ipairs(query:run()) do
  --   local foo = ent:get(Foo)
  --   foo.bar = foo.bar + 1
  --   ent:set(Foo, foo)
  --   print(print_table(foo))
  -- end

  -- local hello = "Hello, world"
  -- for l in lines("Hello\nworld!") do
  --   print("line: "..l)
  -- end

  -- for i, c in ipairs("Hello, world!") do
  --   print(i)
  --   print(c)
  -- end
  -- print("world:\t\t"..tostring(WORLD))
end

print("load script, world: "..tostring(WORLD))


return module
