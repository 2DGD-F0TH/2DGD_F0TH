local vars = {}

function get_vars (meta)
  for k, v in pairs(meta) do
    if type(v) == 'table' and v.t == 'MetaInlines' then
      vars["{{%" .. k .. "%}}"] = {table.unpack(v)}
    end
    if type(v) == 'string' then
      vars["{{%" .. k .. "%}}"] = tostring(v)
    end
  end
end

function find_var (var)
  if vars[var] ~= nil then
    return vars[var]
  else
    return nil
  end
end

function replace (el)
  if vars[el.text] then
    return pandoc.Span(vars[el.text])
  else
    return el
  end
end

return {get_vars=get_vars, replace=replace, find_var=find_var}
