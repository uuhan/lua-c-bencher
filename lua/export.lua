local _M = {}

function _M.add(a, b)
  return a + b
end

local function fib(n)
  if n == 0 then
    return 0
  elseif n == 1 then
    return 1
  else
    return fib(n - 1) + fib(n - 2)
  end
end

_M.fib = fib

return _M
