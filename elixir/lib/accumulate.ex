defmodule Accumulate do
  def accumulate(list, fun) do
    accumulate(list, fun, [])
  end

  defp accumulate([], _fun, acc), do: acc

  defp accumulate([head | tail], fun, acc) do
    accumulate(tail, fun, acc ++ [fun.(head)])
  end
end
